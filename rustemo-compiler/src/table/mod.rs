//! Calculating LR tables
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::{self, Display},
    iter::{self, repeat},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use clap::ValueEnum;
use colored::Colorize;
use itertools::Itertools;
use rustemo::log;

use crate::{
    create_index,
    error::{Error, Result},
    grammar::{Associativity, Priority, Terminal, DEFAULT_PRIORITY},
    index::{
        NonTermIndex, NonTermVec, ProdIndex, ProdVec, StateIndex, StateVec,
        SymbolIndex, SymbolVec, TermIndex, TermVec,
    },
    lang::rustemo_actions::Recognizer,
    settings::{ParserAlgo, Settings},
};

use super::grammar::{res_symbol, Grammar};

#[derive(Debug, Clone)]
pub enum Action {
    Shift(StateIndex),
    Reduce(ProdIndex, usize),
    Accept,
}

/// Specifies the type of the parsing table used during parsing to decide about
/// shift/reduce/goto operations.
#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, PartialEq, Eq, ValueEnum)]
pub enum TableType {
    /// Lookahead LR tables. See
    /// <http://publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-065.pdf>
    LALR,
    /// A slight modification of LALR tables to prevent some reduce/reduce
    /// conflicts. Inspired by <https://doi.org/10.1007/BF00290336>
    #[default]
    LALR_PAGERW,
    /// LALR tables extended with right-nulled entries. Used for GLR parsing
    /// (RNGLR). See <https://doi.org/10.1145/1146809.1146810>
    LALR_RN,
}

type Firsts = BTreeSet<SymbolIndex>;
type FirstSets = SymbolVec<Firsts>;

create_index!(ItemIndex, ItemVec);

/// LR State is a set of LR items and a dict of LR automata actions and gotos.
#[derive(Clone)]
pub struct LRState<'g> {
    grammar: &'g Grammar,

    /// The index of this state.
    pub idx: StateIndex,

    /// The grammar symbol related to this state. Intuitively, the grammar
    /// symbol seen on a transition to this state. E.g. if the symbol is
    /// terminal the parser did a Shift operation to enter this state, otherwise
    /// it did reduce.
    pub symbol: SymbolIndex,

    /// LR(1) items used to construct this state.
    items: ItemVec<LRItem>,

    /// A terminal indexed vector of LR actions. Actions instruct LR parser to
    /// Shift from the input, Reduce the top of the LR stack or accept the
    /// input. For the deterministic parsing the internal vector of actions can
    /// contain only one action.
    pub actions: TermVec<Vec<Action>>,

    /// A non-terminal indexed vector of LR GOTOs. GOTOs represent transitions
    /// to another state after successful reduction of a non-terminal.
    pub gotos: NonTermVec<Option<StateIndex>>,

    /// Terminals and finish flags sorted by the priority and most specific
    /// match (if enabled) for lexical disambiguation. The finish flag, if true,
    /// indicates that if we already have terminals that matched at this
    /// location no further termininals should be tried.
    pub sorted_terminals: Vec<(TermIndex, bool)>,

    /// Each production has a priority. We use this priority to resolve S/R and
    /// R/R conflicts. Since the Shift operation is executed over terminal symbol
    /// to resolve S/R we need terminal priority. But, the priority given for a
    /// terminal directly is used in lexical disambiguation. Instead, we need
    /// terminal priority inherited from productions. We, say that the priority
    /// of terminals in S/R resolution will be the priority of the production
    /// terminal is used in. But, since the same terminal can be used in many
    /// production we will take the maximum for S/R resolution.
    max_prior_for_term: BTreeMap<TermIndex, Priority>,
}

/// Two LR states are equal if they contain the same kernel items.
impl<'g> PartialEq for LRState<'g> {
    fn eq(&self, other: &Self) -> bool {
        let self_ki = self.kernel_items();
        let other_ki = other.kernel_items();
        self_ki.len() == other_ki.len()
            && self_ki.iter().zip(other_ki.iter()).all(|(x, y)| x == y)
    }
}
impl<'g> Eq for LRState<'g> {}

impl<'g> Display for LRState<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\t{}",
            format!(
                "State {}:{}",
                self.idx,
                self.grammar.symbol_name(self.symbol)
            )
            .green()
            .bold(),
            self.items
                .iter()
                .map(|i| i.to_string(self.grammar))
                .collect::<Vec<_>>()
                .join("\n\t")
        )
    }
}

impl<'g> std::fmt::Debug for LRState<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LRState")
            .field("idx", &self.idx)
            .field("symbol", &self.symbol)
            .field("items", &self.items)
            .field("actions", &self.actions)
            .field("gotos", &self.gotos)
            .field("sorted_terminals", &self.sorted_terminals)
            .field("max_prior_for_term", &self.max_prior_for_term)
            .finish()
    }
}

impl<'g> LRState<'g> {
    fn new(
        grammar: &'g Grammar,
        index: StateIndex,
        symbol: SymbolIndex,
    ) -> Self {
        Self {
            grammar,
            idx: index,
            symbol,
            items: ItemVec::new(),
            actions: grammar.new_termvec(vec![]),
            gotos: grammar.new_nontermvec(None),
            max_prior_for_term: BTreeMap::new(),
            sorted_terminals: Vec::new(),
        }
    }

    fn new_with_items(
        grammar: &'g Grammar,
        index: StateIndex,
        symbol: SymbolIndex,
        items: ItemVec<LRItem>,
    ) -> Self {
        Self {
            grammar,
            idx: index,
            symbol,
            items,
            actions: grammar.new_termvec(vec![]),
            gotos: grammar.new_nontermvec(None),
            max_prior_for_term: BTreeMap::new(),
            sorted_terminals: Vec::new(),
        }
    }

    fn add_item(mut self, item: LRItem) -> Self {
        self.items.push(item);
        self
    }

    fn kernel_items(&self) -> Vec<&LRItem> {
        self.items.iter().filter(|i| i.is_kernel()).collect()
    }

    fn nonkernel_items(&self) -> Vec<&LRItem> {
        self.items.iter().filter(|i| !i.is_kernel()).collect()
    }

    /// Closes over LR items of the LRState.
    ///
    /// Starting from the given items (usually just kernel items), for each
    /// item, if right of the dot is a non-terminal, adds all items where LHS is
    /// a given non-terminal and the dot is at the beginning. In other words,
    /// adds all missing non-kernel items.
    fn closure(
        &mut self,
        first_sets: &FirstSets,
        prod_rn_lengths: &Option<ProdVec<usize>>,
    ) {
        loop {
            // This is OK as Hash uses only non-interior-mutable part of the
            // LRItem type.
            #[allow(clippy::mutable_key_type)]
            let mut new_items: BTreeSet<LRItem> = BTreeSet::new();

            for item in &self.items {
                if let Some(symbol) = item.symbol_at_position(self.grammar) {
                    if self.grammar.is_nonterm(symbol) {
                        let mut new_follow;
                        // Find first set of substring that follow symbol at
                        // position
                        if item.position + 1
                            < self.grammar.productions[item.prod].rhs.len()
                        {
                            new_follow = firsts(
                                self.grammar,
                                first_sets,
                                &self.grammar.production_rhs_symbols(item.prod)
                                    [item.position + 1..],
                            );
                            // If symbols that follows the current nonterminal
                            // can derive EMPTY add follows of current item.
                            if new_follow.contains(&self.grammar.empty_index) {
                                new_follow.remove(&self.grammar.empty_index);
                                new_follow.extend(item.follow.borrow().iter());
                            }
                        } else {
                            // If current item position is at the end add all of
                            // its follow to the next item.
                            new_follow = Follow::new();
                            new_follow.extend(item.follow.borrow().iter());
                        }

                        // Get all productions of the current non-terminal and
                        // create LR items with the calculated follow.
                        let nonterm =
                            self.grammar.symbol_to_nonterm_index(symbol);
                        for prod in
                            &self.grammar.nonterminals[nonterm].productions
                        {
                            new_items.insert(LRItem::with_follow(
                                self.grammar,
                                *prod,
                                prod_rn_lengths.as_ref().map(|p| p[*prod]),
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
                match self.items.iter_mut().find(|x| *x == &new_item) {
                    Some(item) => {
                        // Item already exists, update follows
                        let l = item.follow.borrow().len();
                        item.follow
                            .borrow_mut()
                            .extend(new_item.follow.borrow().iter());
                        if item.follow.borrow().len() > l {
                            change = true;
                        }
                    }
                    None => {
                        self.items.push(new_item);
                        change = true;
                    }
                }
            }
            if !change {
                break;
            }
        }
    }

    /// Group LR items per grammar symbol right of the dot, and calculate
    /// terminal max priorities.
    fn group_per_next_symbol(
        &mut self,
    ) -> BTreeMap<SymbolIndex, Vec<ItemIndex>> {
        let mut per_next_symbol: BTreeMap<SymbolIndex, Vec<ItemIndex>> =
            BTreeMap::new();

        for (idx, item) in self.items.iter().enumerate() {
            let symbol = item.symbol_at_position(self.grammar);
            if let Some(symbol) = symbol {
                per_next_symbol.entry(symbol).or_default().push(idx.into());
                if self.grammar.is_term(symbol) {
                    let symbol = self.grammar.symbol_to_term_index(symbol);
                    let prod_prio = self.grammar.productions[item.prod].prio;
                    self.max_prior_for_term
                        .entry(symbol)
                        .and_modify(|v| *v = cmp::max(*v, prod_prio))
                        .or_insert(prod_prio);
                }
            }
        }
        per_next_symbol
    }
}

/// Represents an item in the items set. Item is defined by a production and a
/// position inside production (the dot). If the item is of LR_1 type follow set
/// is also defined. Follow set is a set of terminals that can follow symbol at
/// the given position in the given production.
#[derive(Debug, Eq, Clone, PartialOrd, Ord)]
struct LRItem {
    prod: ProdIndex,

    /// The length of the production
    prod_len: usize,

    /// Right-null production length - the last symbol in the production where
    /// all the following symbols can reduce EMPTY.
    /// Used in RN-GLR to decide when the item can reduce. `None` for LR.
    rn_len: Option<usize>,

    /// The position of "the dot" in the RHS of the production
    position: usize,

    follow: RefCell<Follow>,
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
    #[cfg(test)]
    fn new(grammar: &Grammar, prod: ProdIndex, rn_len: Option<usize>) -> Self {
        LRItem {
            prod,
            prod_len: grammar.production_len(prod),
            rn_len,
            position: 0,
            follow: RefCell::new(Follow::new()),
        }
    }

    fn with_follow(
        grammar: &Grammar,
        prod: ProdIndex,
        rn_len: Option<usize>,
        follow: Follow,
    ) -> Self {
        LRItem {
            prod,
            prod_len: grammar.production_len(prod),
            rn_len,
            position: 0,
            follow: RefCell::new(follow),
        }
    }

    fn symbol_at_position(&self, grammar: &Grammar) -> Option<SymbolIndex> {
        Some(res_symbol(
            grammar.productions.get(self.prod)?.rhs.get(self.position)?,
        ))
    }

    /// Moves position to the right.
    #[inline]
    fn inc_position(mut self) -> Self {
        assert!(self.position < self.prod_len);
        self.position += 1;
        self
    }

    /// True if this item belongs to the kernel core.
    ///
    /// Kernel core items are those where position is not 0 except the augmented
    /// production which by definition belongs to the core.
    #[inline]
    fn is_kernel(&self) -> bool {
        self.position > 0 || self.prod == ProdIndex(0)
    }

    #[inline]
    fn is_reducing(&self) -> bool {
        self.position == self.prod_len
            || match self.rn_len {
                Some(rn_len) => self.position >= rn_len,
                None => false,
            }
    }

    fn to_string(&self, grammar: &Grammar) -> String {
        let prod = &grammar.productions[self.prod];
        let mut rhs = prod
            .rhs_symbols()
            .iter()
            .map(|s| grammar.symbol_name(*s))
            .collect::<Vec<_>>();
        rhs.insert(self.position, ".".into());
        format!(
            "{}: {} = {}    {{{}}}",
            prod.idx,
            grammar
                .symbol_name(grammar.nonterm_to_symbol_index(prod.nonterminal)),
            rhs.join(" "),
            self.follow
                .borrow()
                .iter()
                .map(|f| grammar.symbol_name(*f))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub enum ConflictKind {
    ShiftReduce(ProdIndex),
    ReduceReduce(ProdIndex, ProdIndex),
}

pub struct Conflict<'g, 's> {
    state: &'s LRState<'g>,
    follow: TermIndex,
    kind: ConflictKind,
}

impl<'g, 's> Display for Conflict<'g, 's> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")?;
        Ok(())
    }
}

pub struct LRTable<'g, 's> {
    pub states: StateVec<LRState<'g>>,
    pub layout_state: Option<StateIndex>,
    grammar: &'g Grammar,
    settings: &'s Settings,
    first_sets: FirstSets,

    /// Right-nulled length of productions. Used in RNGLR. RN length is a
    /// position in a production after which all the remaining symbols can
    /// derive EMPTY.
    pub production_rn_lengths: Option<ProdVec<usize>>,
}

impl<'g, 's> LRTable<'g, 's> {
    pub fn new(grammar: &'g Grammar, settings: &'s Settings) -> Result<Self> {
        let first_sets = first_sets(grammar);
        let production_rn_lengths = if settings.table_type == TableType::LALR_RN
        {
            Some(production_rn_lengths(&first_sets, grammar))
        } else {
            None
        };
        let mut table = Self {
            grammar,
            settings,
            states: StateVec::new(),
            layout_state: None,
            first_sets,
            production_rn_lengths,
        };

        table.check_empty_sets()?;

        table.calc_states(grammar.augmented_index);
        if let Some(augmented_layout_index) = grammar.augmented_layout_index {
            table.layout_state = Some(StateIndex(table.states.len()));
            table.calc_states(augmented_layout_index)
        }

        log!("LR states constructed. Updating follows.");
        table.propagate_follows();

        log!(
            "Calculate REDUCTION entries in ACTION tables and resolve \
            possible conflicts."
        );
        table.calculate_reductions();

        log!("Sort terminals for lexical disambiguation");
        table.sort_terminals();

        if settings.print_table {
            println!("LR TABLE:");
            println!("{}", table);
        }

        Ok(table)
    }

    /// Calculate LR states with GOTOs and ACTIONs for the given Grammar.
    ///
    /// This collection of states is used to generate LR/GLR parser tables.
    fn calc_states(&mut self, start_symbol: SymbolIndex) {
        let mut current_state_idx = self.states.len();

        let prods = &self.grammar.symbol_to_nonterm(start_symbol).productions;
        assert_eq!(prods.len(), 1);

        // Create a state for the first production (augmented)
        let state = LRState::new(
            self.grammar,
            StateIndex(current_state_idx),
            start_symbol,
        )
        .add_item(LRItem::with_follow(
            self.grammar,
            prods[0],
            self.production_rn_lengths.as_ref().map(|p| p[prods[0]]),
            Follow::from([self.grammar.stop_index]),
        ));
        current_state_idx += 1;

        // States to be processed.
        let mut state_queue = VecDeque::from([state]);

        log!("Calculating LR automaton states.");
        while let Some(mut state) = state_queue.pop_front() {
            // For each state calculate its closure first, i.e. starting from a so
            // called "kernel items" expand collection with non-kernel items. We
            // will also calculate GOTO and ACTIONS dicts for each state. These
            // dicts will be keyed by a grammar symbol.
            state.closure(&self.first_sets, &self.production_rn_lengths);

            // To find out other states we examine following grammar symbols in the
            // current state (symbols following current position/"dot") and group
            // all items by a grammar symbol.
            let per_next_symbol = state.group_per_next_symbol();

            // Create accept action if possible.
            for &symbol in per_next_symbol.keys() {
                if symbol == self.grammar.stop_index {
                    state.actions[self.grammar.symbol_to_term_index(symbol)] =
                        vec![Action::Accept];
                    break;
                }
            }

            // Create new states reachable from the current state.
            let new_states =
                Self::create_new_states(self.grammar, &state, per_next_symbol);

            // Find states that already exists and try to merge. If not possible to
            // merge or not found push state to state queue.
            for mut new_state in new_states {
                let mut new_state_found = true;
                let mut target_state_symbol = new_state.symbol;
                let mut target_state_idx = StateIndex(current_state_idx);
                if let Some(old_state) = self
                    .states
                    .iter_mut()
                    .chain(state_queue.iter_mut())
                    .chain(iter::once(&mut state))
                    .find(|x| **x == new_state)
                {
                    // If the same state already exists try to merge.
                    if Self::merge_state(self.settings, old_state, &new_state) {
                        new_state_found = false;
                        target_state_symbol = old_state.symbol;
                        target_state_idx = old_state.idx;
                    }
                }

                // Create GOTO for non-terminal or Shift Action for terminal.
                if self.grammar.is_nonterm(target_state_symbol) {
                    state.gotos[self
                        .grammar
                        .symbol_to_nonterm_index(target_state_symbol)] =
                        Some(target_state_idx);
                } else {
                    let term =
                        self.grammar.symbol_to_term_index(new_state.symbol);
                    state.actions[term].push(Action::Shift(target_state_idx));
                }

                if new_state_found {
                    // Merge is not possible. Create new state.
                    new_state.idx = StateIndex(current_state_idx);

                    state_queue.push_back(new_state);
                    current_state_idx += 1;
                }
            }

            self.states.push(state);
        }
    }

    /// Try to merge new_state to old_state if possible. If not possible return
    /// false.
    ///
    /// If old state has no R/R conflicts additional check is made and merging is
    /// not done if it would add R/R conflict.
    fn merge_state(
        settings: &Settings,
        old_state: &mut LRState<'g>,
        new_state: &LRState<'g>,
    ) -> bool {
        // States with different kernel sets cannot be merged.
        if old_state != new_state {
            return false;
        }

        let old_state_items = old_state
            .items
            .clone()
            .into_iter()
            .filter(|item| item.is_kernel());

        // Item pairs of item from an old state and corresponding item from the new state.
        let item_pairs: Vec<(&mut LRItem, &LRItem)> = iter::zip(
            old_state.items.iter_mut().filter(|item| item.is_kernel()),
            old_state_items
                .map(|x| new_state.items.iter().find(|&i| *i == x).unwrap()),
        )
        .collect();

        if settings.table_type != TableType::LALR {
            // If this is not pure LALR check to see if merging would introduce R/R.
            // In case it would, do not merge but keep these states split.
            for (old, new) in &item_pairs {
                if !old.is_reducing() {
                    continue;
                }
                for (old_in, new_in) in &item_pairs {
                    if old == old_in {
                        continue;
                    }
                    // Check if any of the current follow terminals exists in any other
                    // new follow but not in the same item old follow.
                    if old.follow.borrow().iter().any(|x| {
                        new_in.follow.borrow().contains(x)
                            && !old_in.follow.borrow().contains(x)
                            && !new.follow.borrow().contains(x) // If conflict exist in new, merge anyway
                    }) {
                        return false;
                    }
                }
            }
        }

        // Do the merge by updating old items follow sets.
        for (old, new) in item_pairs {
            old.follow.borrow_mut().extend(new.follow.borrow().iter())
        }
        true
    }

    /// Propagate LR items follows.
    ///
    /// This is needed due to state merging. Whenever merge occurs, target state
    /// follows might get updated so we have to propagate those changes to other
    /// states.
    fn propagate_follows(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for state in self.states.iter_mut() {
                // Refresh closure to propagate follows from kernel items to
                // non-kernel of the same state as the merge is done only for kernel
                // items.
                state.closure(&self.first_sets, &self.production_rn_lengths);
            }

            for state in self.states.iter() {
                // Use GOTOs and ACTIONS to propagate follows between states.
                state
                    .gotos
                    .iter()
                    .filter_map(|x| x.as_ref())
                    .chain(state.actions.iter().flat_map(|x| {
                        x.iter().filter_map(|a| match a {
                            Action::Shift(state) => Some(state),
                            _ => None,
                        })
                    }))
                    .for_each(|&target_state| {
                        for target_item in &mut self.states[target_state]
                            .items
                            .iter()
                            .filter(|x| x.is_kernel())
                        {
                            // Find corresponding item in state
                            if let Some(source_item) =
                                state.items.iter().find(|&x| {
                                    x.prod == target_item.prod
                                        && x.position
                                            == target_item.position - 1
                                })
                            {
                                // Update follow of target item with item from state
                                let follow_len =
                                    target_item.follow.borrow().len();
                                target_item
                                    .follow
                                    .borrow_mut()
                                    .extend(source_item.follow.borrow().iter());

                                // if target item follow was changed set changed to true
                                if target_item.follow.borrow().len()
                                    > follow_len
                                {
                                    changed = true
                                }
                            }
                        }
                    })
            }
        }
    }

    /// Calculate reductions entries in action tables and resolve possible
    /// conflicts.
    fn calculate_reductions(&mut self) {
        let mut aug_symbols = vec![self.grammar.augmented_index];
        if let Some(layout_index) = self.grammar.augmented_layout_index {
            aug_symbols.push(layout_index);
        }
        for state in &mut self.states {
            for item in state.items.iter().filter(|x| x.is_reducing()) {
                let prod = &self.grammar.productions[item.prod];

                // Accept if reducing by augmented productions for STOP lookahead
                if aug_symbols.contains(
                    &self.grammar.nonterm_to_symbol_index(prod.nonterminal),
                ) {
                    let actions = &mut state.actions[TermIndex(0)];
                    actions.push(Action::Accept);
                    continue;
                }

                let new_reduce = Action::Reduce(item.prod, item.position);
                for follow_symbol in item.follow.borrow().iter() {
                    let follow_term =
                        self.grammar.symbol_to_term(*follow_symbol);
                    let actions = &mut state.actions[follow_term.idx];
                    if actions.is_empty() {
                        // No other action are possible for this follow terminal.
                        // Just register this reduction.
                        actions.push(new_reduce.clone());
                    } else {
                        // Conflict. Try to resolve.
                        let (shifts, reduces): (Vec<_>, Vec<_>) =
                            actions.clone().into_iter().partition(|x| {
                                matches!(x, Action::Shift(_) | Action::Accept)
                            });
                        // Only one SHIFT or ACCEPT might exists for a single
                        // terminal but many REDUCEs might exist.
                        assert!(shifts.len() <= 1);

                        let mut should_reduce = true;
                        if let Some(shift) = shifts.first() {
                            // Shift/Reduce conflict. Use assoc and priority to
                            // resolve. For disambiguation treat ACCEPT action the
                            // same as SHIFT.
                            let shift_prio = match shift {
                                Action::Accept => DEFAULT_PRIORITY,
                                _ => state.max_prior_for_term[&follow_term.idx],
                            };
                            match prod.prio.cmp(&shift_prio) {
                                Ordering::Less => {
                                    // If priority of existing SHIFT action is
                                    // higher then leave it instead
                                    should_reduce = false
                                }
                                Ordering::Equal => {
                                    // If priorities are the same use associativity
                                    // Terminals associativity has priority over
                                    // production associativity
                                    match (&prod.assoc, &follow_term.assoc) {
                                        (
                                            Associativity::Left,
                                            Associativity::None,
                                        )
                                        | (_, Associativity::Right) => {
                                            // Override SHIFT with this REDUCE
                                            assert!(actions.len() == 1);
                                            actions.pop();
                                        }
                                        (
                                            Associativity::Right,
                                            Associativity::None,
                                        )
                                        | (_, Associativity::Left) => {
                                            // If associativity is right leave SHIFT
                                            // action as "stronger" and don't consider
                                            // this reduction any more. Right
                                            // associative reductions can't be in the
                                            // same set of actions together with SHIFTs.
                                            should_reduce = false;
                                        }
                                        (
                                            Associativity::None,
                                            Associativity::None,
                                        ) => {
                                            // If priorities are the same and no
                                            // associativity defined use preferred
                                            // strategy.
                                            let empty = prod.rhs.is_empty();
                                            let prod_pse = empty
                                                && self
                                                    .settings
                                                    .prefer_shifts_over_empty
                                                && !prod.nopse;
                                            let prod_ps = !empty
                                                && self.settings.prefer_shifts
                                                && !prod.nops;
                                            should_reduce =
                                                !(prod_pse || prod_ps);
                                        }
                                    }
                                }
                                Ordering::Greater => {
                                    // This item operation priority is higher =>
                                    // override with reduce
                                    assert!(actions.len() == 1);
                                    actions.pop();
                                }
                            }
                        }

                        if should_reduce {
                            if reduces.is_empty() {
                                actions.push(new_reduce.clone())
                            } else {
                                // REDUCE/REDUCE conflicts.
                                // Try to resolve using priorities.
                                let reduces_prio = reduces
                                    .iter()
                                    .map(|x| match x {
                                        Action::Reduce(prod, ..) => {
                                            self.grammar.productions[*prod].prio
                                        }
                                        other => panic!(
                                            "This should not happen. Got {:?}",
                                            other
                                        ),
                                    })
                                    .collect::<Vec<_>>();
                                if reduces_prio.iter().all(|x| prod.prio < *x) {
                                    // Current product priority is less than all
                                    // other reductions. Do not add this reduction.
                                } else if reduces_prio
                                    .iter()
                                    .all(|x| prod.prio > *x)
                                {
                                    // Current product priority is greater than all
                                    // other reductions. This reduction should
                                    // replace all others.
                                    actions.retain(|x| {
                                        !matches!(x, Action::Reduce(..))
                                    });
                                    actions.push(new_reduce.clone())
                                } else {
                                    // For LR parsing non-empty reductions are
                                    // preferred over empty...
                                    if let ParserAlgo::LR =
                                        self.settings.parser_algo
                                    {
                                        // ... so remove all empty reductions.
                                        actions.retain(|x| !matches!(x, Action::Reduce(_, len) if *len == 0));

                                        if item.prod_len > 0
                                            || actions.is_empty()
                                        {
                                            // If current reduction is non-empty add it.
                                            actions.push(new_reduce.clone())
                                        }
                                    } else {
                                        // This R/R conflict can't be resolved.
                                        // Just add the new reduction and GLR
                                        // will handle it by investigating all
                                        // possibilities.
                                        actions.push(new_reduce.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Sort terminals for each state according to explicit priority and
    /// terminal recognizer type. String recognizers have precedence over regex
    /// recognizers if most specific match strategy is enabled. For most
    /// specific match, longer string recognizers have precedence over shorter.
    fn sort_terminals(&mut self) {
        for state in &mut self.states {
            let mut terminals = state
                .actions
                .iter()
                .enumerate()
                .filter(|(_, actions)| !actions.is_empty())
                .map(|(idx, _)| self.grammar.term_by_index(TermIndex(idx)))
                .collect::<Vec<_>>();

            let term_prio = |term: &Terminal| -> u32 {
                term.prio * 1000
                    + if self.settings.lexical_disamb_most_specific {
                        match &term.recognizer {
                            Some(recognizer) => {
                                (match recognizer {
                                    Recognizer::StrConst(str_rec) => {
                                        str_rec.as_ref().len()
                                    }
                                    Recognizer::RegexTerm(_) => 0,
                                }) as u32
                            }
                            None => 0,
                        }
                    } else {
                        0
                    }
            };
            terminals.sort_by(|&l, &r| {
                let l_term_prio = term_prio(l);
                let r_term_prio = term_prio(r);
                r_term_prio.cmp(&l_term_prio)
            });

            log!(
                "SORTED: {:?}",
                terminals.iter().map(|t| &t.name).collect::<Vec<_>>()
            );

            // Calculate "finish" flags
            let mut sorted_terminals: Vec<(TermIndex, bool)> = vec![];
            let mut last_prio = None;
            for terminal in terminals {
                let finish = self.settings.lexical_disamb_most_specific
                    && matches!(
                        terminal.recognizer,
                        Some(Recognizer::StrConst(_))
                    );
                let last_finish =
                    last_prio.is_some_and(|prio| terminal.prio != prio);
                last_prio = Some(terminal.prio);
                if let Some(t) = sorted_terminals.last_mut() {
                    t.1 |= last_finish
                }
                sorted_terminals.push((terminal.idx, finish));
            }

            state.sorted_terminals = sorted_terminals;
        }
    }

    /// Create new states that can be reached from the given state.
    fn create_new_states(
        grammar: &'g Grammar,
        state: &LRState,
        per_next_symbol: BTreeMap<SymbolIndex, Vec<ItemIndex>>,
    ) -> Vec<LRState<'g>> {
        let mut states = Vec::new();
        for (symbol, items) in per_next_symbol {
            let next_state_items = items
                .into_iter()
                .map(|i| state.items[i].clone().inc_position())
                .collect();
            states.push(LRState::new_with_items(
                grammar,
                StateIndex(0), // Temporary value. The caller will set the real index.
                symbol,
                next_state_items,
            ));
        }
        states
    }

    /// Check for states with GOTO links but without SHIFT links.
    ///
    /// This is invalid as GOTO links will never be traversed.
    fn check_empty_sets(&self) -> Result<()> {
        if let Some((idx, _)) = self
            .first_sets
            .iter()
            .enumerate()
            .find(|(_, s)| s.is_empty())
        {
            return Err(Error::Error(format!(
                "First set empty for grammar symbol {:?}.\n\
                 An infinite recursion on the grammar symbol.",
                &self.grammar.symbol_name(SymbolIndex(idx))
            )));
        }
        Ok(())
    }

    pub fn get_conflicts(&'s self) -> Vec<Conflict<'g, 's>> {
        self.states.iter().flat_map(|state| {
            state
                .actions
                .iter()
                .enumerate()
                .filter(|(_, actions)| actions.len() > 1)
                .flat_map(|(idx, actions)| {
                    actions.iter().combinations(2).map(move |conflict| (idx, conflict))
                })
                .map(|(term_index, conflict)| {
                    let kind = match &conflict[..] {
                        [Action::Shift(_), Action::Reduce(prod, _)]
                            | [Action::Reduce(prod, _), Action::Shift(_)]=>
                                ConflictKind::ShiftReduce(*prod),
                        [Action::Reduce(prod1, _), Action::Reduce(prod2, _)] =>
                            ConflictKind::ReduceReduce(*prod1,  *prod2),
                        e => {
                            // This cannot happen as we have combinations of size 2.
                            print!("{e:?}");
                            unreachable!()
                        }
                    };
                    Conflict {
                        state,
                        follow: TermIndex(term_index),
                        kind
                    }
                })
        }).collect()
    }

    pub fn print_conflicts_report(&self, conflicts: &Vec<Conflict<'g, 's>>) {
        for conflict in conflicts {
            println!("{} {}", "In".green().bold(), conflict.state);
            print!(
                "When I saw {} and see token {} ahead I can't decide",
                self.grammar.symbol_name(conflict.state.symbol).green(),
                self.grammar
                    .symbol_name(
                        self.grammar.term_to_symbol_index(conflict.follow)
                    )
                    .green()
            );
            match conflict.kind {
                ConflictKind::ShiftReduce(prod) => {
                    println!(
                        " should I shift or reduce by production:\n{}\n",
                        self.grammar.productions[prod]
                            .to_string(self.grammar)
                            .green()
                    );
                }
                ConflictKind::ReduceReduce(prod1, prod2) => {
                    println!(
                        " should I reduce by production:\n{}\nor production:\n{}\n",
                        self.grammar.productions[prod1].to_string(self.grammar).green(),
                        self.grammar.productions[prod2].to_string(self.grammar).green()
                    );
                }
            }
        }
        let shift_reduce_len = conflicts
            .iter()
            .filter(|c| matches!(c.kind, ConflictKind::ShiftReduce(..)))
            .count();
        let reduce_reduce_len = conflicts
            .iter()
            .filter(|c| matches!(c.kind, ConflictKind::ReduceReduce(..)))
            .count();
        println!(
            "{} conflict(s). {} Shift/Reduce and {} Reduce/Reduce.",
            shift_reduce_len + reduce_reduce_len,
            shift_reduce_len,
            reduce_reduce_len
        );
    }

    /// Maximal number of actions per state/token. For LR can't be >1.
    #[inline]
    pub fn max_actions(&self) -> usize {
        self.states
            .iter()
            .map(|state| {
                state.actions.iter().map(|a| a.len()).max().unwrap_or(0)
            })
            .max()
            .unwrap()
    }

    #[inline]
    pub fn max_recognizers(&self) -> usize {
        self.states
            .iter()
            .map(|state| state.actions.iter().filter(|a| !a.is_empty()).count())
            .max()
            .unwrap()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from(
            r#"
            digraph grammar {
            rankdir=LR
            fontname = "Bitstream Vera Sans"
            fontsize = 8
            node[
                shape=record,
                style=filled,
                fillcolor=aliceblue
            ]
            nodesep = 0.3
            edge[dir=black,arrowtail=empty]

        "#,
        );

        let dot_escape = |s: &String| {
            s.replace('\n', r"\n")
                .replace('\\', "\\\\")
                .replace('"', r#"\""#)
                .replace('|', r"\|")
                .replace('{', r"\{")
                .replace('}', r"\}")
                .replace('>', r"\>")
                .replace('<', r"\<")
                .replace('?', r"\?")
        };

        for state in &self.states {
            let mut kernel_items_str = String::new();
            for item in state.kernel_items() {
                kernel_items_str += &format!(
                    "{}\\l",
                    dot_escape(&item.to_string(self.grammar))
                );
            }

            let nonkernel_items = state.nonkernel_items();
            let mut nonkernel_items_str = if !nonkernel_items.is_empty() {
                String::from("|")
            } else {
                String::new()
            };

            for item in nonkernel_items {
                nonkernel_items_str += &format!(
                    "{}\\l",
                    dot_escape(&item.to_string(self.grammar))
                );
            }

            let mut reductions: Vec<String> = vec![];
            for term in &self.grammar.terminals {
                let mut term_reduction_prods: Vec<String> = vec![];
                for action in &state.actions[term.idx] {
                    match action {
                        Action::Shift(target_state_idx) => {
                            dot += &format!(
                                "{} -> {} [label=\"SHIFT:{}\"]\n",
                                state.idx, target_state_idx, term.name
                            )
                        }
                        Action::Reduce(prod_idx, len) => {
                            term_reduction_prods
                                .push(format!("({},{})", prod_idx, len));
                        }
                        Action::Accept => {
                            dot += &format!(
                                "{} -> ACCEPT [label=\"{}\"]\n",
                                state.idx, term.name
                            )
                        }
                    }
                }
                if !term_reduction_prods.is_empty() {
                    dbg!(&term_reduction_prods);
                    let r = term_reduction_prods.join(", ");
                    reductions.push(if term_reduction_prods.len() > 1 {
                        format!("{}:[{}]", dot_escape(&term.name), r)
                    } else {
                        format!("{}:{}", dot_escape(&term.name), r)
                    });
                }
            }
            let reductions = if !reductions.is_empty() {
                format!("|Reductions:\\l{}", reductions.join(", "))
            } else {
                String::new()
            };

            dot += &format!(
                "{} [label=\"{}|{}{}{}\"]\n",
                state.idx,
                dot_escape(&format!(
                    "{}:{}",
                    state.idx,
                    self.grammar.symbol_name(state.symbol)
                )),
                kernel_items_str,
                nonkernel_items_str,
                reductions
            );

            // GOTOs
            for nonterm in &self.grammar.nonterminals {
                if let Some(target_state_idx) = state.gotos[nonterm.idx] {
                    dot += &format!(
                        "{} -> {} [label=\"GOTO:{}\"]\n",
                        state.idx, target_state_idx, nonterm.name
                    )
                }
            }
        }
        dot += "\n}\n";
        dot
    }
}

fn production_rn_lengths(
    first_sets: &SymbolVec<BTreeSet<SymbolIndex>>,
    grammar: &Grammar,
) -> ProdVec<usize> {
    let mut prod_rn_lens = ProdVec::new();
    for production in &grammar.productions {
        let mut rn_len = production.rhs.len();
        for symbol in production.rhs_symbols().iter().rev() {
            if first_sets[*symbol].contains(&grammar.empty_index) {
                rn_len -= 1;
            } else {
                break;
            }
        }
        prod_rn_lens.push(rn_len)
    }
    prod_rn_lens
}

impl<'g, 's> Display for LRTable<'g, 's> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for state in &self.states {
            writeln!(
                f,
                "\nState {}:{}",
                state.idx,
                self.grammar.symbol_name(state.symbol),
            )?;
            let actions = state
                .actions
                .iter()
                .enumerate()
                .filter(|(_, a)| !a.is_empty())
                .flat_map(|(i, a)| repeat(i).zip(a.iter()))
                .map(|(i, a)| {
                    (
                        self.grammar.terminals[TermIndex(i)].name.clone(),
                        match a {
                            Action::Shift(s) => format!("Shift to {s}"),
                            Action::Reduce(p, l) => {
                                format!(
                                    "Reduce for len {l} by:   {}",
                                    self.grammar.productions[*p]
                                        .to_string(self.grammar)
                                )
                            }
                            Action::Accept => "Accept".into(),
                        },
                    )
                })
                .collect::<Vec<_>>();

            if !actions.is_empty() {
                writeln!(f, "\tACTIONS:")?;
                for (term, action) in actions {
                    writeln!(f, "\t\t{term} => {action}")?;
                }
            }

            // GOTOs
            let gotos = state
                .gotos
                .iter()
                .enumerate()
                .filter(|(_, g)| g.is_some())
                .map(|(idx, g)| {
                    let g = g.unwrap();
                    (
                        self.grammar.nonterminals[NonTermIndex(idx)]
                            .name
                            .clone(),
                        format!(
                            "State {}:{}",
                            self.grammar.symbol_name(self.states[g].symbol),
                            g.0
                        ),
                    )
                })
                .collect::<Vec<_>>();

            if !gotos.is_empty() {
                writeln!(f, "\tGOTOs:")?;
                for (nonterm, goto) in gotos {
                    writeln!(f, "\t\t{nonterm} => {goto}")?;
                }
            }
        }
        Ok(())
    }
}

/// Calculates the sets of terminals that can start the sentence derived from all
/// grammar symbols.
///
/// The Dragon book p. 221.
fn first_sets(grammar: &Grammar) -> FirstSets {
    let mut first_sets = SymbolVec::new();

    // First set for each terminal contains only the terminal itself.
    for terminal in &grammar.terminals {
        let mut new_set = Firsts::new();
        new_set.insert(terminal.idx.symbol_index());
        first_sets.push(new_set);
    }

    // Initialize empty sets for nonterminals
    grammar
        .nonterminals
        .iter()
        .for_each(|_| first_sets.push(Firsts::new()));

    // EMPTY derives EMPTY
    first_sets[grammar.empty_index].insert(grammar.empty_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in &grammar.productions {
            let lhs_nonterm =
                grammar.nonterm_to_symbol_index(production.nonterminal);

            let rhs_firsts =
                firsts(grammar, &first_sets, &production.rhs_symbols());

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
/// Currently unused
type Follow = BTreeSet<SymbolIndex>;
#[allow(dead_code)]
type FollowSets = SymbolVec<Follow>;
#[cfg(test)]
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
        for production in &grammar.productions {
            let lhs_symbol =
                grammar.nonterm_to_symbol_index(production.nonterminal);

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

#[cfg(test)]
mod tests {

    use std::cell::RefCell;
    use std::collections::BTreeSet;

    use crate::index::{ProdIndex, StateIndex, SymbolIndex};
    use crate::table::{
        first_sets, follow_sets, ItemIndex, LRTable, TableType,
    };
    use crate::{
        grammar::Grammar,
        output_cmp,
        settings::Settings,
        table::{Follow, LRItem},
    };

    use super::{production_rn_lengths, LRState};

    fn follow<T, I>(indexes: T) -> BTreeSet<SymbolIndex>
    where
        T: IntoIterator<Item = I>,
        I: Into<SymbolIndex>,
    {
        indexes.into_iter().map(|i| i.into()).collect()
    }

    fn test_grammar() -> Grammar {
        r#"
        E: T Ep;
        Ep: "+" T Ep | EMPTY;
        T: F Tp;
        Tp: "*" F Tp | EMPTY;
        F: "(" E ")" | "id";

        terminals
        Plus: "+";
        Mul: "*";
        LParen: "(";
        RParen: ")";
        id: "id";
        "#
        .parse()
        .unwrap()
    }

    fn test_grammar_2() -> Grammar {
        r#"
        E: E "+" T | T;
        T: T "*" F | F;
        F: "(" E ")" | "id";

        terminals
        Plus: "+";
        Mul: "*";
        LParen: "(";
        RParen: ")";
        id: "id";
        "#
        .parse()
        .unwrap()
    }

    /// Grammar from the Dragon book, p.278
    /// This grammar is LR(1) but not LALR.
    /// See also: https://www.gnu.org/software/bison/manual/bison.html#Mysterious-Conflicts
    fn test_non_lalr_grammar() -> Grammar {
        r#"
        S: A "a" | "b" A "c" | B "c" | "b" B "a";
        A: "d";
        B: "d";
        terminals
        a_t: "a";
        b_t: "b";
        c_t: "c";
        d_t: "d";
        "#
        .parse()
        .unwrap()
    }

    fn test_ambiguous_grammar() -> Grammar {
        r#"
        E: E "+" E {1, left}
            | E "*" E {2, left}
            | E "^" E {3, right}
            | "(" E ")"
            | "id";

        terminals
        Plus: "+";
        Mul: "*";
        Power: "^";
        LParen: "(";
        RParen: ")";
        id: "id";
        "#
        .parse()
        .unwrap()
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
            &follow(grammar.symbol_indexes(&["LParen", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("T")],
            &follow(grammar.symbol_indexes(&["LParen", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("E")],
            &follow(grammar.symbol_indexes(&["LParen", "id"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Ep")],
            &follow(grammar.symbol_indexes(&["Plus", "EMPTY"]))
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Tp")],
            &follow(grammar.symbol_indexes(&["Mul", "EMPTY"]))
        );
    }

    #[test]
    fn test_follow_sets() {
        let grammar = test_grammar();
        let follow_sets = follow_sets(&grammar, &first_sets(&grammar));

        assert_eq!(
            &follow_sets[grammar.symbol_index("E")],
            &follow(grammar.symbol_indexes(&["RParen", "STOP"]))
        );
        dbg!(grammar
            .symbol_names(follow_sets[grammar.symbol_index("Ep")].clone()));
        assert_eq!(
            &follow_sets[grammar.symbol_index("Ep")],
            &follow(grammar.symbol_indexes(&["RParen", "STOP"]))
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("T")],
            &follow(grammar.symbol_indexes(&["Plus", "RParen", "STOP"]))
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("Tp")],
            &follow(grammar.symbol_indexes(&["Plus", "RParen", "STOP"]))
        );
    }

    #[test]
    fn test_prooduction_rn_lengths() {
        let grammar = test_grammar();
        let first_sets = first_sets(&grammar);

        let production_rn_lengths =
            production_rn_lengths(&first_sets, &grammar);

        // RN length of "E: T Ep" is 1 as "Ep" can derive EMPTY but "T" can't.
        assert_eq!(production_rn_lengths[ProdIndex(1)], 1);

        // RN length of "Ep: "+" T Ep" is 2.
        assert_eq!(production_rn_lengths[ProdIndex(2)], 2);
    }

    #[test]
    fn test_symbol_at_position() {
        let grammar = test_grammar();

        let prod = ProdIndex(1);
        let mut item = LRItem::new(&grammar, prod, None);
        assert_eq!(
            &grammar.symbol_names(grammar.productions[prod].rhs_symbols()),
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
                    rn_len: None,
                    position: 1,
                    follow: RefCell::new(Follow::new()),
                })
                .add_item(LRItem {
                    prod: 2.into(),
                    prod_len: grammar.production_len(2.into()),
                    rn_len: None,
                    position: 1,
                    follow: RefCell::new(Follow::new()),
                })
                .add_item(LRItem {
                    prod: 3.into(),
                    prod_len: grammar.production_len(2.into()),
                    rn_len: None,
                    position: 1,
                    follow: RefCell::new(Follow::new()),
                })
                .add_item(LRItem {
                    prod: 4.into(),
                    prod_len: grammar.production_len(3.into()),
                    rn_len: None,
                    position: 2,
                    follow: RefCell::new(Follow::new()),
                });

        let per_next_symbol = lr_state.group_per_next_symbol();

        // log!("Symbols: {:#?}", grammar.symbol_names(per_next_symbol.keys()));
        // Symbols: ["+", "*", ")"]
        //log!("Pernext: {:?}", per_next_symbol);
        // Pernext: {SymbolIndex(1): [ItemIndex(0)], SymbolIndex(2): [ItemIndex(1)], SymbolIndex(4): [ItemIndex(2)]}

        // Check items grouping per symbol
        assert_eq!(per_next_symbol.len(), 4);
        assert_eq!(
            per_next_symbol.keys().cloned().collect::<Vec<_>>(),
            [1, 2, 3, 5]
                .iter()
                .map(|v| SymbolIndex(*v))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            per_next_symbol.values().cloned().collect::<Vec<_>>(),
            vec![
                vec![0.into()],
                vec![1.into()],
                vec![2.into()],
                vec![3.into()]
            ]
        );

        // Check production based term priorities
        assert_eq!(
            lr_state.max_prior_for_term
                [&grammar.symbol_to_term_index(grammar.term_by_name["Power"])],
            3
        );
        assert_eq!(
            lr_state.max_prior_for_term
                [&grammar.symbol_to_term_index(grammar.term_by_name["Mul"])],
            2
        );
        assert_eq!(
            lr_state.max_prior_for_term
                [&grammar.symbol_to_term_index(grammar.term_by_name["Plus"])],
            1
        );
    }

    #[test]
    fn test_merge_states() {
        let grammar = test_grammar();
        let lr_item_1 = LRItem {
            prod: ProdIndex(1),
            prod_len: 2,
            rn_len: None,
            position: 2,
            follow: RefCell::new(Follow::new()),
        };
        let lr_item_2 = LRItem {
            prod: ProdIndex(2),
            prod_len: 3,
            rn_len: None,
            position: 3,
            follow: RefCell::new(Follow::new()),
        };
        let old_state = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: RefCell::new(follow([1, 3])),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: RefCell::new(follow([2])),
                ..lr_item_2
            });

        // This should be merged as there are no introduced R/R conflicts
        let new_state_1 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: RefCell::new(follow([1])),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: RefCell::new(follow([2, 4])),
                ..lr_item_2
            });
        let mut old_state_1 = old_state.clone();
        let settings = Settings::default();
        assert!(LRTable::merge_state(
            &settings,
            &mut old_state_1,
            &new_state_1
        ));
        // When the merge succeed verify that items follows are indeed extended.
        assert_eq!(
            *old_state_1.items[ItemIndex(0)].follow.borrow(),
            follow([1, 3])
        );
        assert_eq!(
            *old_state_1.items[ItemIndex(1)].follow.borrow(),
            follow([2, 4])
        );

        // This merge introduces new R/R conflict as the second item has 1 in
        // the follow set. Term 1 exists in the first item of the old state so
        // merging will make two items eligible for reduction on the term 1 in
        // the input.
        let new_state_2 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: RefCell::new(follow([3])),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: RefCell::new(follow([2, 1])),
                ..lr_item_2
            });
        let mut old_state_2 = old_state.clone();
        assert!(!LRTable::merge_state(
            &settings,
            &mut old_state_2,
            &new_state_2
        ));
        // Verify that no merge happened
        assert_eq!(
            *old_state_2.items[ItemIndex(0)].follow.borrow(),
            follow([1, 3])
        );
        assert_eq!(
            *old_state_2.items[ItemIndex(1)].follow.borrow(),
            follow([2])
        );

        // The last thing to check is situation where new state has R/R
        // conflicts and there are no additional merge introduced R/R conflicts.
        // This time we should merge as the R/R conflict is not introduced by
        // merge process but exists due to the grammar not being LR(1).
        let new_state_3 = LRState::new(&grammar, 0.into(), 0.into())
            .add_item(LRItem {
                follow: RefCell::new(follow([1, 3])),
                ..lr_item_1
            })
            .add_item(LRItem {
                follow: RefCell::new(follow([2, 1])),
                ..lr_item_2
            });
        let mut old_state_3 = old_state.clone();
        assert!(LRTable::merge_state(
            &settings,
            &mut old_state_3,
            &new_state_3
        ));
        // Verify that no merge happened
        assert_eq!(
            *old_state_3.items[ItemIndex(0)].follow.borrow(),
            follow([1, 3])
        );
        assert_eq!(
            *old_state_3.items[ItemIndex(1)].follow.borrow(),
            follow([2, 1])
        );
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
                    None,
                    follow([grammar.stop_index]),
                ));

        lr_state.closure(&firsts, &None);

        let prods = [1, 4, 7, 8];
        let follow_sets = [
            grammar.symbol_indexes(&["STOP"]),
            grammar.symbol_indexes(&["STOP", "Plus"]),
            grammar.symbol_indexes(&["STOP", "Plus", "Mul"]),
            grammar.symbol_indexes(&["STOP", "Plus", "Mul"]),
        ];

        assert_eq!(lr_state.items.len(), 4);

        itertools::izip!(&lr_state.items, prods, follow_sets).for_each(
            |(item, prod, follows)| {
                assert_eq!(item.prod, prod.into());
                assert!(item.follow.borrow().iter().eq(follows.iter()));
            },
        );

        log!("{:?}", lr_state);
    }

    #[test]
    fn test_lr_states_for_grammar_2() {
        let grammar = test_grammar_2();

        let settings = Settings {
            table_type: TableType::LALR,
            ..Settings::default()
        };

        let table = LRTable::new(&grammar, &settings).unwrap();
        output_cmp!(
            "src/table/grammar_2.expected",
            format!("{:#?}", table.states)
        );
    }

    #[test]
    fn test_lr_states_for_non_lalr_grammar() {
        let grammar = test_non_lalr_grammar();

        // Calculating LR tables with LALR method will result in a state with
        // R/R conflicts. So, deterministic LR parsing method cannot be used for
        // this grammar and LALR construction method.
        //
        // Conflicts are found in state 2 which is entered when 'd' is
        // recognized in the input. There are two R/R conflicts, for inputs 'a'
        // and 'c'. In both case parser may reduce both A and B.
        let settings = Settings {
            table_type: TableType::LALR,
            ..Settings::default()
        };

        let table = LRTable::new(&grammar, &settings).unwrap();

        output_cmp!(
            "src/table/grammar_nonlalr_lalr.expected",
            format!("{grammar}\n\n{:#?}", table.states)
        );

        // In LALR_PAGERW construction method R/R conflicts are avoided during
        // merge phase where states are kept split if merging would introduce
        // new R/R conflict. This essentially makes LALR_PAGERW very close in
        // power to canonical LR(1) but with the number of states which is
        // almost like in LALR (i.e. LR(0)).
        //
        // In this case we have 13 states while in previous LALR case there was
        // 12 states.
        let settings = Settings {
            table_type: TableType::LALR_PAGERW,
            ..Settings::default()
        };

        let table = LRTable::new(&grammar, &settings).unwrap();

        output_cmp!(
            "src/table/grammar_nonlalr_lalr_pagerw.expected",
            format!("{grammar}\n\n{:#?}", table.states)
        );
    }

    #[test]
    fn test_sorted_terminals() {
        let grammar: Grammar = r#"
            S: A | C | B;
            terminals
            A: /\d+/;
            B: "bb";
            C: "c";
            "#
        .parse()
        .unwrap();

        let settings = Settings {
            table_type: TableType::LALR_PAGERW,
            ..Settings::default()
        };

        let table = LRTable::new(&grammar, &settings).unwrap();
        assert_eq!(
            &table.states[StateIndex(0)]
                .sorted_terminals
                .iter()
                .map(|i| i.0 .0)
                .collect::<Vec<_>>(),
            &vec![2, 3, 1]
        );
    }
}
