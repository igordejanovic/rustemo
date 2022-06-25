//! Calculating LR tables

use std::{
    cmp,
    collections::{BTreeMap, BTreeSet},
    iter,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use rustemort::{
    create_index,
    index::{
        NonTermVec, ProdIndex, StateIndex, SymbolIndex, SymbolVec, TermIndex,
        TermVec,
    },
    log,
    lr::Action,
};

use crate::grammar::Priority;

use super::grammar::{res_symbol, Grammar};

type Follow = BTreeSet<SymbolIndex>;
type Firsts = BTreeSet<SymbolIndex>;

fn follow<T, I>(indexes: T) -> BTreeSet<SymbolIndex>
where
    T: IntoIterator<Item = I>,
    I: Into<SymbolIndex>,
{
    indexes.into_iter().map(|i| i.into()).collect()
}

type FollowSets = SymbolVec<Follow>;
type FirstSets = SymbolVec<Firsts>;

create_index!(ItemIndex, ItemVec);

/// LR State is a set of LR items and a dict of LR automata actions and gotos.
#[derive(Debug, Clone)]
struct LRState {
    /// The index of this state.
    idx: StateIndex,

    /// The grammar symbol related to this state. Intuitively, the grammar
    /// symbol seen on a transition to this state. E.g. if the symbol is
    /// terminal the parser did a Shift operation to enter this state, otherwise
    /// it did reduce.
    symbol: SymbolIndex,

    /// LR(1) items used to construct this state.
    items: ItemVec<LRItem>,

    /// A terminal indexed vector of LR actions. Actions instruct LR parser to
    /// Shift from the input, Reduce the top of the LR stack or accept the
    /// input. For the deterministic parsing the internal vector of actions can
    /// contain only one action.
    actions: TermVec<Vec<Action>>,

    /// A non-terminal indexed vector of LR GOTOs. GOTOs represent transitions
    /// to another state after successful reduction of a non-terminal.
    gotos: NonTermVec<Option<StateIndex>>,

    // Each production has a priority. We use this priority to resolve S/R and
    // R/R conflicts. Since the Shift operation is executed over terminal symbol
    // to resolve S/R we need terminal priority. But, the priority given for a
    // terminal directly is used in lexical disambiguation. Instead, we need
    // terminal priority inherited from productions. We, say that the priority
    // of terminals in S/R resolution will be the priority of the production
    // terminal is used in. But, since the same terminal can be used in many
    // production we will take the maximum for S/R resolution.
    max_prior_for_term: BTreeMap<TermIndex, Priority>,
}

/// Two LR states are equal if they contain the same kernel items.
impl PartialEq for LRState {
    fn eq(&self, other: &Self) -> bool {
        let self_ki = self.kernel_items();
        let other_ki = other.kernel_items();
        self_ki.len() == other_ki.len()
            && self_ki.iter().zip(other_ki.iter()).all(|(x, y)| x == y)
    }
}
impl Eq for LRState {}

impl LRState {
    fn new(grammar: &Grammar, index: StateIndex, symbol: SymbolIndex) -> Self {
        Self {
            idx: index,
            symbol,
            items: ItemVec::new(),
            actions: grammar.new_termvec(vec![Action::Error]),
            gotos: grammar.new_nontermvec(None),
            max_prior_for_term: BTreeMap::new(),
        }
    }

    fn new_with_items(
        grammar: &Grammar,
        index: StateIndex,
        symbol: SymbolIndex,
        items: ItemVec<LRItem>,
    ) -> Self {
        Self {
            idx: index,
            symbol,
            items,
            actions: grammar.new_termvec(vec![Action::Error]),
            gotos: grammar.new_nontermvec(None),
            max_prior_for_term: BTreeMap::new(),
        }
    }

    fn add_item(mut self, item: LRItem) -> Self {
        self.items.push(item);
        self
    }

    fn kernel_items(&self) -> Vec<&LRItem> {
        self.items.iter().filter(|i| i.is_kernel()).collect()
    }
}

/// Represents an item in the items set. Item is defined by a production and a
/// position inside production (the dot). If the item is of LR_1 type follow set
/// is also defined. Follow set is a set of terminals that can follow symbol at
/// the given position in the given production.
#[derive(Debug, Eq, Clone, PartialOrd, Ord)]
struct LRItem {
    prod: ProdIndex,
    prod_len: usize,
    position: usize,
    follow: Follow,
}

impl std::hash::Hash for LRItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.prod.hash(state);
        self.position.hash(state);
    }
}

impl PartialEq for LRItem {
    fn eq(&self, other: &Self) -> bool {
        self.prod == other.prod && self.position == other.position
    }
}

/// LRItem is a production with a dot in the RHS.
///
/// Intuitively, the dot signifies the position where the parsing process is in
/// a given state. The beginning position is 0, before the first symbol in a
/// production RHS. The end position is len(RHS), after the last symbol in a
/// RHS.
///
/// LRItem also keeps a set of follow terminals. The item is valid only if the
/// production is followed by a terminal from the given follow set.
///
/// # Example
///
/// ```rust
/// // If prod with index 5 is A: B a C;
/// let item = LRItem::new(5)
///                 .next_item().unwrap()
///                 .next_item().unwrap();
/// assert_eq(&item.position, 2)
/// ```
///
/// ```text
/// A: B a . C;
///        ^
///        |------ position is 2
/// ```
impl LRItem {
    fn new(grammar: &Grammar, prod: ProdIndex) -> Self {
        LRItem {
            prod,
            prod_len: grammar.production_len(prod),
            position: 0,
            follow: Follow::new(),
        }
    }

    fn with_follow(grammar: &Grammar, prod: ProdIndex, follow: Follow) -> Self {
        LRItem {
            prod,
            prod_len: grammar.production_len(prod),
            position: 0,
            follow,
        }
    }

    fn add_follow(mut self, symbol: SymbolIndex) -> Self {
        self.follow.insert(symbol);
        self
    }

    fn symbol_at_position(&self, grammar: &Grammar) -> Option<SymbolIndex> {
        Some(res_symbol(
            grammar
                .productions
                .as_ref()?
                .get(self.prod)
                .unwrap()
                .rhs
                .get(self.position)?,
        ))
    }

    fn next_item(&self, grammar: &Grammar) -> Option<Self> {
        if self.position < grammar.production_len(self.prod) {
            Some(Self {
                prod: self.prod,
                prod_len: grammar.production_len(self.prod),
                position: self.position + 1,
                follow: self.follow.clone(),
            })
        } else {
            None
        }
    }

    /// Moves position to the right.
    fn inc_position(mut self) -> Self {
        assert!(self.position < self.prod_len);
        self.position += 1;
        self
    }

    /// True if this item belongs to the kernel core.
    ///
    /// Kernel core items are those where position is not 0 except the augmented
    /// production which by definition belongs to the core.
    fn is_kernel(&self) -> bool {
        self.position > 0 || self.prod == ProdIndex(0)
    }

    fn is_reducing(&self) -> bool {
        self.position == self.prod_len
    }
}

/// Calculate LR states with GOTOs and ACTIONs for the given Grammar.
///
/// This collection of states is used to generate LR/GLR parser tables.
fn lr_states_for_grammar(grammar: &Grammar) -> Vec<LRState> {
    let first_sets = first_sets(grammar);
    check_empty_sets(grammar, &first_sets);

    let follow_sets = follow_sets(grammar, &first_sets);

    // Create a state for the first production (augmented)
    let state = LRState::new(grammar, StateIndex(0), grammar.augmented_index)
        .add_item(LRItem::with_follow(grammar, ProdIndex(0), Follow::new()));

    // States to be processed.
    let mut state_queue = vec![state];
    // Finished states.
    let mut states = vec![];

    let mut current_state_idx: usize = 1;

    log!("Calculating LR automaton states.");
    while let Some(mut state) = state_queue.pop() {
        // For each state calculate its closure first, i.e. starting from a so
        // called "kernel items" expand collection with non-kernel items. We
        // will also calculate GOTO and ACTIONS dicts for each state. These
        // dicts will be keyed by a grammar symbol.
        closure(&mut state, grammar, &first_sets);

        // To find out other states we examine following grammar symbols in the
        // current state (symbols following current position/"dot") and group
        // all items by a grammar symbol.
        let per_next_symbol = group_per_next_symbol(grammar, &mut state);

        // Create accept action if possible.
        for (&symbol, _) in &per_next_symbol {
            if symbol == grammar.stop_index {
                state.actions[grammar.symbol_to_term(symbol)] =
                    vec![Action::Accept];
                break;
            }
        }

        // Create new states reachable from the current state.
        let new_states = create_new_states(&grammar, &state, per_next_symbol);

        // Find states that already exists and try to merge. If not possible to
        // merge or not found push state to state queue.
        for mut new_state in new_states {
            let mut new_state_found = true;
            let mut target_state = &new_state;
            let mut target_state_idx = StateIndex(current_state_idx);
            if let Some(mut old_state) = states
                .iter_mut()
                .chain(state_queue.iter_mut())
                .find(|x| **x == new_state)
            {
                // If the same state already exists try to merge.
                if merge_state(&mut old_state, &new_state) {
                    new_state_found = false;
                    target_state = old_state;
                    target_state_idx = old_state.idx;
                }
            }

            // Create GOTO for non-terminal or Shift Action for terminal.
            if grammar.is_nonterm(target_state.symbol) {
                state.gotos[grammar.symbol_to_nonterm(target_state.symbol)] =
                    Some(target_state_idx);
            } else {
                let term = grammar.symbol_to_term(new_state.symbol);
                state.actions[term]
                    .push(Action::Shift(target_state_idx, term));
            }

            if new_state_found {
                // Merge is not possible. Create new state.
                new_state.idx = StateIndex(current_state_idx);

                state_queue.push(new_state);
                current_state_idx += 1;
            }
        }

        states.push(state);
    }

    log!("LR states constructed. Updating follows.");
    propagate_follows();

    log!(
        "Calculate REDUCTION entries in ACTION tables and resolve \
          possible conflicts."
    );
    calculate_reductions();

    states
}

/// Try to merge new_state to old_state if possible. If not possible return
/// false.
///
/// If old state has no R/R conflicts additional check is made and merging is
/// not done if it would add R/R conflict.
fn merge_state(old_state: &mut LRState, new_state: &LRState) -> bool {
    // States with different kernel sets cannot be merged.
    if old_state != new_state {
        return false;
    }

    let old_state_items = old_state
        .items
        .clone()
        .into_iter()
        .filter(|item| item.is_reducing());

    // Item pairs of item from an old state and corresponding item from the new state.
    let item_pairs: Vec<(&mut LRItem, &LRItem)> = iter::zip(
        old_state.items.iter_mut().filter(|item| item.is_reducing()),
        old_state_items
            .map(|x| new_state.items.iter().find(|&i| *i == x).unwrap()),
    )
    .collect();

    for (old, new) in &item_pairs {
        for (old_in, new_in) in &item_pairs {
            if old == old_in {
                continue;
            }
            // Check if any of the current follow terminals exists in any other
            // new follow but not in the same item old follow.
            if old
                .follow
                .iter()
                .find(|&x| {
                    new_in.follow.contains(x)
                        && !old_in.follow.contains(x)
                        && !new.follow.contains(x) // If conflict exist in new, merge anyway
                })
                .is_some()
            {
                return false;
            }
        }
    }

    // Do the merge by updating old items follow sets.
    for (old, new) in item_pairs {
        old.follow.extend(new.follow.iter())
    }
    true
}

/// Update follow sets by propagation for each LR item.
fn propagate_follows() {
    todo!()
}

/// Calculate reductions entries in action tables and resolve possible
/// conflicts.
fn calculate_reductions() {
    todo!()
}

/// Create new states that can be reached from the given state.
fn create_new_states(
    grammar: &Grammar,
    state: &LRState,
    per_next_symbol: BTreeMap<SymbolIndex, Vec<ItemIndex>>,
) -> Vec<LRState> {
    let mut states = Vec::new();
    for (symbol, items) in per_next_symbol {
        let next_state_items = items
            .into_iter()
            .map(|i| state.items[i].clone())
            .map(|i| i.inc_position())
            .collect();
        states.push(LRState::new_with_items(
            &grammar,
            StateIndex(0), // Temporary value. The caller will set the real index.
            symbol,
            next_state_items,
        ));
    }
    states
}

fn find_state<'a>(
    state: &LRState,
    states: &'a [LRState],
) -> Option<&'a LRState> {
    states.iter().find(|x| *x == state)
}

/// Group LR items per grammar symbol right of the dot, and calculate
/// terminal max priorities.
fn group_per_next_symbol(
    grammar: &Grammar,
    state: &mut LRState,
) -> BTreeMap<SymbolIndex, Vec<ItemIndex>> {
    let mut per_next_symbol = BTreeMap::new();

    for (idx, item) in state.items.iter().enumerate() {
        let symbol = item.symbol_at_position(&grammar);
        if let Some(symbol) = symbol {
            per_next_symbol
                .entry(symbol)
                .or_insert(vec![])
                .push(idx.into());
            if grammar.is_term(symbol) {
                let symbol = grammar.symbol_to_term(symbol);
                let prod_prio = grammar.productions()[item.prod].prio;
                state
                    .max_prior_for_term
                    .entry(symbol)
                    .and_modify(|v| *v = cmp::max(*v, prod_prio))
                    .or_insert(prod_prio);
            }
        }
    }
    per_next_symbol
}

/// Check for states with GOTO links but without SHIFT links.
///
/// This is invalid as GOTO links will never be traversed.
fn check_empty_sets(grammar: &Grammar, first_sets: &FirstSets) {
    first_sets
        .iter()
        .enumerate()
        .filter(|(_, s)| s.is_empty())
        .for_each(|(idx, _)| {
            panic!(
                "First set empty for grammar symbol {:?}.\n\
                 An infinite recursion on the grammar symbol.",
                &grammar.symbol_name(SymbolIndex(idx))
            )
        });
}

/// Calculates the sets of terminals that can start the sentence derived from all
/// grammar symbols.
///
/// The Dragon book p. 221.
fn first_sets(grammar: &Grammar) -> FirstSets {
    let mut first_sets = SymbolVec::new();

    // First set for each terminal contains only the terminal itself.
    if let Some(ref terminals) = grammar.terminals {
        for terminal in terminals {
            let mut new_set = Firsts::new();
            new_set.insert(terminal.idx.to_symbol_index());
            first_sets.push(new_set);
        }
    }

    // Initialize empty sets for nonterminals
    if let Some(ref nonterminals) = grammar.nonterminals {
        nonterminals
            .iter()
            .for_each(|_| first_sets.push(Firsts::new()));
    }

    // EMPTY derives EMPTY
    first_sets[grammar.empty_index].insert(grammar.empty_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in grammar.productions.as_ref().unwrap() {
            let lhs_nonterm = grammar.nonterm_to_symbol(production.nonterminal);

            let rhs_firsts =
                firsts(&grammar, &first_sets, &production.rhs_symbols());

            let lhs_len = first_sets[lhs_nonterm].len();

            first_sets[lhs_nonterm].extend(rhs_firsts);

            // Check if any addition is actually performed.
            if lhs_len < first_sets[lhs_nonterm].len() {
                additions = true
            }
        }
    }
    first_sets
}

/// For the given sequence of symbols finds a set of FIRST terminals.
///
/// Finds all terminals which can start the given sequence of symbols. Note that
/// if all symbols in the sequence can derive EMPTY, EMPTY will be a part of the
/// returned set.
fn firsts(
    grammar: &Grammar,
    first_sets: &FirstSets,
    symbols: &[SymbolIndex],
) -> Firsts {
    let mut firsts = Firsts::new();
    let mut break_out = false;
    for &symbol in symbols {
        let symbol_firsts = &first_sets[symbol];
        let mut empty = false;

        for first in symbol_firsts {
            if *first == grammar.empty_index {
                empty = true;
            } else {
                firsts.insert(*first);
            }
        }

        // We should proceed to the next symbol in sequence only if the current
        // symbol can produce EMPTY.
        if !empty {
            break_out = true;
            break;
        }
    }
    if !break_out {
        // If we reached the end of symbol sequence and each symbol along the
        // way could derive EMPTY than we must add EMPTY to the firsts.
        firsts.insert(grammar.empty_index);
    }
    firsts
}

/// Calculates the sets of terminals that can follow some non-terminal for the
/// given grammar.
///
/// The dragon book p.221
fn follow_sets(grammar: &Grammar, first_sets: &FirstSets) -> FollowSets {
    let mut follow_sets = FollowSets::new();
    for _ in 0..first_sets.len() {
        follow_sets.push(Follow::new());
    }

    // Rule 1: Place $ in FOLLOW(S), where S is the start symbol, and $ is
    // the input right endmarker.
    follow_sets[grammar.augmented_index].insert(grammar.stop_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in grammar.productions.as_ref().unwrap() {
            let lhs_symbol = grammar.nonterm_to_symbol(production.nonterminal);

            // Rule 2: If there is a production A -> α B β then everything in
            // FIRST(β) except EMPTY is in FOLLOW(B).
            for idx in 0..production.rhs.len() {
                let rhs_symbol = production.rhs_symbol(idx);
                let elements = follow_sets[rhs_symbol].len();
                let mut break_out = false;
                for rnext in &production.rhs[idx + 1..] {
                    let follow_symbols = &first_sets[res_symbol(rnext)];

                    follow_sets[rhs_symbol].extend(
                        follow_symbols
                            .iter()
                            .filter(|&&s| s != grammar.empty_index),
                    );

                    if !follow_symbols.contains(&grammar.empty_index) {
                        break_out = true;
                        break;
                    }
                }

                if !break_out {
                    // Rule 3: If all symbols right of current RHS produce EMPTY
                    // then this RHS symbol must contain all what follows symbol
                    // at LHS.
                    let lhs_follows: Follow =
                        follow_sets[lhs_symbol].iter().copied().collect();
                    follow_sets[rhs_symbol].extend(lhs_follows.iter());
                }

                if follow_sets[rhs_symbol].len() > elements {
                    additions = true
                }
            }
        }
    }
    follow_sets
}

/// Closes over LR items of the given LRState.
///
/// Starting from the given items (usually just kernel items), for each item, if
/// right of the dot is a non-terminal, adds all items where LHS is a given
/// terminal and the dot is at the beginning. In other words, adds all missing
/// non-kernel items.
fn closure(state: &mut LRState, grammar: &Grammar, first_sets: &FirstSets) {
    loop {
        let mut new_items: BTreeSet<LRItem> = BTreeSet::new();

        for item in &state.items {
            if let Some(symbol) = item.symbol_at_position(grammar) {
                if grammar.is_nonterm(symbol) {
                    let mut new_follow;
                    // Find first set of substring that follow symbol at position
                    if item.position + 1
                        < grammar.productions()[item.prod].rhs.len()
                    {
                        new_follow = firsts(
                            &grammar,
                            &first_sets,
                            &grammar.production_rhs_symbols(item.prod)
                                [item.position + 1..],
                        );
                        // If symbols that follows the current nonterminal can
                        // derive EMPTY add follows of current item.
                        if new_follow.contains(&grammar.empty_index) {
                            new_follow.remove(&grammar.empty_index);
                            new_follow.extend(&item.follow);
                        }
                    } else {
                        // If current item position is at the end add all of its
                        // follow to the next item.
                        new_follow = Follow::new();
                        new_follow.extend(&item.follow);
                    }

                    // Get all productions of the current non-terminal and
                    // create LR items with the calculated follow.
                    let nonterm = grammar.symbol_to_nonterm(symbol);
                    for prod in &grammar.nonterminals()[nonterm].productions {
                        new_items.insert(LRItem::with_follow(
                            &grammar,
                            *prod,
                            new_follow.clone(),
                        ));
                    }
                }
            }
        }

        // Add all new items to state.items. If item is already there update
        // follow. If there is no change break from the loop.
        let mut change = false;
        for new_item in new_items {
            match state.items.iter_mut().find(|x| *x == &new_item) {
                Some(item) => {
                    // Item already exists, update follows
                    let l = item.follow.len();
                    item.follow.extend(&new_item.follow);
                    if item.follow.len() > l {
                        change = true;
                    }
                }
                None => {
                    state.items.push(new_item);
                    change = true;
                }
            }
        }
        if !change {
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::BTreeSet, iter};

    use crate::{
        grammar::Grammar,
        rustemo::RustemoParser,
        table::{first_sets, Follow, ItemIndex, LRItem},
    };
    use rustemort::{
        index::{ProdIndex, StateIndex, SymbolIndex},
        log,
    };

    use super::{
        closure, follow, follow_sets, group_per_next_symbol, merge_state,
        LRState,
    };

    fn test_grammar() -> Grammar {
        RustemoParser::default().parse(
            r#"
            E: T Ep;
            Ep: "+" T Ep | EMPTY;
            T: F Tp;
            Tp: "*" F Tp | EMPTY;
            F: "(" E ")" | "id";
            "#
            .into(),
        )
    }

    fn test_ambiguous_grammar() -> Grammar {
        RustemoParser::default().parse(
            r#"
            E: E "+" E {1, left}
             | E "*" E {2, left}
             | "(" E ")"
             | "id";
            "#
            .into(),
        )
    }

    #[test]
    fn test_first_sets() {
        let grammar = test_grammar();
        let first_sets = first_sets(&grammar);

        assert_eq!(first_sets.len(), 13);

        // First of terminal is just a terminal itself.
        assert_eq!(
            &first_sets[grammar.symbol_index("id")],
            &follow(grammar.symbol_indexes(&["id"]))
        );

        assert_eq!(
            &first_sets[grammar.symbol_index("F")],
            &follow(grammar.symbol_indexes(&["(", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("T")],
            &follow(grammar.symbol_indexes(&["(", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("E")],
            &follow(grammar.symbol_indexes(&["(", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Ep")],
            &follow(grammar.symbol_indexes(&["+", "EMPTY"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Tp")],
            &follow(grammar.symbol_indexes(&["*", "EMPTY"]))
        );
    }

    #[test]
    fn test_follow_sets() {
        let grammar = test_grammar();
        let follow_sets = follow_sets(&grammar, &first_sets(&grammar));

        assert_eq!(
            &follow_sets[grammar.symbol_index("E")],
            &follow(grammar.symbol_indexes(&[")", "STOP"]))
        );
        dbg!(grammar.symbol_names(&follow_sets[grammar.symbol_index("Ep")]));
        assert_eq!(
            &follow_sets[grammar.symbol_index("Ep")],
            &follow(grammar.symbol_indexes(&[")", "STOP"]))
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("T")],
            &follow(grammar.symbol_indexes(&["+", ")", "STOP"]))
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("Tp")],
            &follow(grammar.symbol_indexes(&["+", ")", "STOP"]))
        );
    }

    #[test]
    fn test_symbol_at_position() {
        let grammar = test_grammar();

        let prod = ProdIndex(1);
        let mut item = LRItem::new(&grammar, prod);
        assert_eq!(
            &grammar.symbol_names(
                &grammar.productions.as_ref().unwrap()[prod].rhs_symbols()
            ),
            &["T", "Ep"]
        );
        assert_eq!(
            item.symbol_at_position(&grammar).unwrap(),
            grammar.symbol_index("T")
        );
        item.position = 1;
        assert_eq!(
            &grammar.symbol_name(item.symbol_at_position(&grammar).unwrap()),
            "Ep"
        );
        item.position = 2;
        assert!(item.symbol_at_position(&grammar).is_none());
        item.position = 3;
        assert!(item.symbol_at_position(&grammar).is_none());
    }

    #[test]
    fn test_group_per_next_symbol() {
        let grammar = test_ambiguous_grammar();

        // Create some LR state
        let mut lr_state =
            LRState::new(&grammar, 0.into(), grammar.symbol_index("E"))
                .add_item(LRItem {
                    prod: 1.into(),
                    prod_len: grammar.production_len(1.into()),
                    position: 1,
                    follow: Follow::new(),
                })
                .add_item(LRItem {
                    prod: 2.into(),
                    prod_len: grammar.production_len(2.into()),
                    position: 1,
                    follow: Follow::new(),
                })
                .add_item(LRItem {
                    prod: 3.into(),
                    prod_len: grammar.production_len(3.into()),
                    position: 2,
                    follow: Follow::new(),
                });

        let per_next_symbol = group_per_next_symbol(&grammar, &mut lr_state);

        //log!("Symbols: {:?}", grammar.symbol_names(per_next_symbol.keys()));
        // Symbols: ["+", "*", ")"]
        //log!("Pernext: {:?}", per_next_symbol);
        // Pernext: {SymbolIndex(1): [ItemIndex(0)], SymbolIndex(2): [ItemIndex(1)], SymbolIndex(4): [ItemIndex(2)]}

        // Check items grouping per symbol
        assert_eq!(per_next_symbol.len(), 3);
        assert_eq!(
            per_next_symbol.keys().cloned().collect::<Vec<_>>(),
            vec![1, 2, 4]
                .iter()
                .map(|v| SymbolIndex(*v))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            per_next_symbol.values().cloned().collect::<Vec<_>>(),
            vec![vec![0.into()], vec![1.into()], vec![2.into()]]
        );

        // Check production based term priorities
        assert_eq!(
            lr_state.max_prior_for_term
                [&grammar.symbol_to_term(grammar.term_by_name["*"])],
            2
        );
        assert_eq!(
            lr_state.max_prior_for_term
                [&grammar.symbol_to_term(grammar.term_by_name["+"])],
            1
        );
    }

    #[test]
    fn test_merge_states() {
        let grammar = test_grammar();
        let lr_item_1 = LRItem {
            prod: ProdIndex(1),
            prod_len: 2,
            position: 2,
            follow: Follow::new(),
        };
        let lr_item_2 = LRItem {
            prod: ProdIndex(2),
            prod_len: 3,
            position: 3,
            follow: Follow::new(),
        };
        let old_state = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: follow([1, 3]),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: follow([2]),
                ..lr_item_2
            });

        // This should be merged as there are no introduced R/R conflicts
        let new_state_1 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: follow([1]),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: follow([2, 4]),
                ..lr_item_2
            });
        let mut old_state_1 = old_state.clone();
        assert!(merge_state(&mut old_state_1, &new_state_1));
        // When the merge succeed verify that items follows are indeed extended.
        assert_eq!(old_state_1.items[0.into()].follow, follow([1, 3]));
        assert_eq!(old_state_1.items[1.into()].follow, follow([2, 4]));

        // This merge introduces new R/R conflict as the second item has 1 in
        // the follow set. Term 1 exists in the first item of the old state so
        // merging will make two items eligible for reduction on the term 1 in
        // the input.
        let new_state_2 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: follow([3]),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: follow([2, 1]),
                ..lr_item_2
            });
        let mut old_state_2 = old_state.clone();
        assert!(!merge_state(&mut old_state_2, &new_state_2));
        // Verify that no merge happened
        assert_eq!(old_state_2.items[0.into()].follow, follow([1, 3]));
        assert_eq!(old_state_2.items[1.into()].follow, follow([2]));

        // The last thing to check is situation where new state has R/R
        // conflicts and there are no additional merge introduced R/R conflicts.
        // This time we should merge as the R/R conflict is not introduced by
        // merge process but exists due to the grammar not being LR(1).
        let new_state_3 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: follow([1, 3]),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: follow([2, 1]),
                ..lr_item_2
            });
        let mut old_state_3 = old_state.clone();
        assert!(merge_state(&mut old_state_3, &new_state_3));
        // Verify that no merge happened
        assert_eq!(old_state_3.items[0.into()].follow, follow([1, 3]));
        assert_eq!(old_state_3.items[1.into()].follow, follow([2, 1]));
    }

    #[test]
    fn test_closure() {
        let grammar = test_grammar();
        let firsts = first_sets(&grammar);

        // Create some LR state
        let mut lr_state =
            LRState::new(&grammar, StateIndex(0), grammar.symbol_index("T"))
                .add_item(LRItem::with_follow(
                    &grammar,
                    ProdIndex(1),
                    follow([grammar.stop_index]),
                ));

        closure(&mut lr_state, &grammar, &firsts);

        let prods = [1, 4, 7, 8];
        let follow_sets = [
            grammar.symbol_indexes(&["STOP"]),
            grammar.symbol_indexes(&["STOP", "+"]),
            grammar.symbol_indexes(&["STOP", "+", "*"]),
            grammar.symbol_indexes(&["STOP", "+", "*"]),
        ];

        assert_eq!(lr_state.items.len(), 4);

        itertools::izip!(&lr_state.items, prods, follow_sets)
            .into_iter()
            .for_each(|(item, prod, follows)| {
                assert_eq!(item.prod, prod.into());
                assert!(item.follow.iter().eq(follows.iter()));
            });

        log!("{:?}", lr_state);
    }
}
