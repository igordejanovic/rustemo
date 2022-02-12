//! Calculating LR tables

use std::collections::HashSet;

use indexmap::IndexMap;

use crate::{
    grammar::Priority,
    index::{NonTermVec, ProdIndex, StateIndex, SymbolIndex, SymbolVec, TermVec},
    parser::Action,
};

use super::grammar::{res_symbol, Grammar};

type Follow = HashSet<SymbolIndex>;
type FollowSets = SymbolVec<Follow>;
type Firsts = HashSet<SymbolIndex>;
type FirstSets = SymbolVec<Firsts>;

/// LR State is a set of LR items and a dict of LR automata actions and gotos.
struct LRState {
    state: StateIndex,
    symbol: SymbolIndex,
    items: Vec<LRItem>,
    actions: TermVec<Action>,
    gotos: NonTermVec<Option<StateIndex>>,
}

impl LRState {
    fn new(grammar: &Grammar, state: StateIndex, symbol: SymbolIndex) -> Self {
        Self {
            state,
            symbol,
            items: vec![],
            actions: grammar.new_termvec(Action::Error),
            gotos: grammar.new_nontermvec(None),
        }
    }

    fn add_item(&mut self, item: LRItem) -> &Self {
        self.items.push(item);
        self
    }
}

/// Represents an item in the items set. Item is defined by a production and a
/// position inside production (the dot). If the item is of LR_1 type follow set
/// is also defined. Follow set is a set of terminals that can follow symbol at
/// the given position in the given production.
#[derive(Eq)]
struct LRItem {
    prod: ProdIndex,
    position: usize,
    follow: Follow,
}

impl PartialEq for LRItem {
    fn eq(&self, other: &Self) -> bool {
        self.prod == other.prod && self.position == other.position
    }
}

impl LRItem {
    fn new(prod: ProdIndex) -> Self {
        LRItem {
            prod,
            position: 0,
            follow: Follow::new(),
        }
    }

    fn new_follow(prod: ProdIndex, follow: Follow) -> Self {
        LRItem {
            prod,
            position: 0,
            follow,
        }
    }

    fn add_follow(&mut self, symbol: SymbolIndex) -> &Self {
        self.follow.insert(symbol);
        self
    }

    fn symbol_at_position(&self, grammar: &Grammar) -> Option<SymbolIndex> {
        Some(res_symbol(
            grammar
                .productions
                .as_ref()?
                .get(self.prod)?
                .rhs
                .get(self.position)?,
        ))
    }

    fn next_item(&self, grammar: &Grammar) -> Option<Self> {
        if self.position < grammar.productions.as_ref()?[self.prod].rhs.len() {
            Some(Self {
                prod: self.prod,
                position: self.position + 1,
                follow: self.follow.clone(),
            })
        } else {
            None
        }
    }
}

pub(in crate::lang) struct LRTable {}

pub(in crate::lang) fn calculate_lr_tables(grammar: Grammar) {
    let first_sets = first_sets(&grammar);
    check_empty_sets(&grammar, &first_sets);
    let follow_sets = follow_sets(&grammar, &first_sets);

    let mut state = LRState::new(&grammar, StateIndex(0), grammar.start_index);
    state.add_item(LRItem::new(ProdIndex(0)));

    let mut state_queue = vec![state];
    let mut states = vec![];

    while let Some(mut state) = state_queue.pop() {
        // For each state calculate its closure first, i.e. starting from a so
        // called "kernel items" expand collection with non-kernel items. We
        // will also calculate GOTO and ACTIONS dicts for each state. These
        // dicts will be keyed by a grammar symbol.
        closure(&mut state, &grammar, &first_sets);
        states.push(state);
        let state = states.last().unwrap();

        // To find out other states we examine following grammar symbols in the
        // current state (symbols following current position/"dot") and group
        // all items by a grammar symbol.
        let mut per_next_symbol = IndexMap::new();

        // Each production has a priority. But since productions are grouped by
        // grammar symbol that is ahead we take the maximal priority given for
        // all productions for the given grammar symbol.
        let mut max_prior_per_symbol: IndexMap<SymbolIndex, Priority> = IndexMap::new();

        for item in &state.items {
            let symbol = item.symbol_at_position(&grammar);
            if let Some(symbol) = symbol {
                per_next_symbol.entry(symbol).or_insert(vec![]).push(item);
            }
        }
    }
}

/// Check for states with GOTO links but without SHIFT links.
/// This is invalid as the GOTO link will never be traversed.
fn check_empty_sets(grammar: &Grammar, first_sets: &SymbolVec<HashSet<SymbolIndex>>) {
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
/// grammar symbols. The Dragon book p. 221.
fn first_sets(grammar: &Grammar) -> FirstSets {
    let mut first_sets = SymbolVec::new();

    if let Some(ref terminals) = grammar.terminals {
        for terminal in terminals {
            let mut new_set = Firsts::new();
            new_set.insert(terminal.idx.to_symbol_index());
            first_sets.push(new_set);
        }
    }
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

            let lhs_len = first_sets[lhs_nonterm].len();
            let rhs_firsts = firsts(
                &grammar,
                &first_sets,
                production.rhs.iter().map(|assgn| res_symbol(assgn)).collect(),
            );

            first_sets[lhs_nonterm].extend(rhs_firsts);

            // Check if any addition is actuall performed.
            if lhs_len < first_sets[lhs_nonterm].len() {
                additions = true
            }
        }
    }
    first_sets
}

/// For a given collection of symbols find a set of FIRST terminals. If all
/// `symbols` symbols can derive EMPTY add EMPTY to the output.
fn firsts(grammar: &Grammar, first_sets: &FirstSets, symbols: Vec<SymbolIndex>) -> Firsts {
    let mut firsts = Firsts::new();
    let mut break_out = false;
    for symbol in symbols {
        let symbol_firsts = &first_sets[symbol];
        let mut empty = false;

        let firsts_addition = symbol_firsts
            .iter()
            .filter(|&x| {
                if *x == grammar.empty_index {
                    empty = true;
                    false
                } else {
                    true
                }
            })
            .copied()
            .collect::<Firsts>();
        firsts.extend(firsts_addition);

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
    follow_sets[grammar.start_index].insert(grammar.stop_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in grammar.productions.as_ref().unwrap() {
            let lhs_symbol = grammar.nonterm_to_symbol(production.nonterminal);

            // Rule 2: If there is a production A -> α B β then everything in
            // FIRST(β) except EMPTY is in FOLLOW(B).
            for (idx, assign) in production.rhs[..production.rhs.len() - 1]
                .iter()
                .enumerate()
            {
                let rhs_symbol = res_symbol(assign);
                let elements = follow_sets[rhs_symbol].len();
                let mut break_out = false;
                for rassign in &production.rhs[idx + 1..] {
                    let follow_symbols = &first_sets[res_symbol(rassign)];

                    follow_sets[rhs_symbol]
                        .extend(follow_symbols.iter().filter(|&&s| s != grammar.empty_index));

                    if !follow_symbols.contains(&grammar.empty_index) {
                        break_out = true;
                        break;
                    }
                }

                if !break_out {
                    // All symbols right of current RHS produce EMPTY, thus,
                    // according to Rule 3 this RHS symbol must contain all that
                    // follows symbol at LHS.
                    let lhs_follows: Follow = follow_sets[lhs_symbol].iter().copied().collect();
                    follow_sets[rhs_symbol].extend(lhs_follows.iter());
                }

                if follow_sets[rhs_symbol].len() > elements {
                    additions = true
                }
            }
            // At the end handle situation A -> α B where FOLLOW(B) should
            // contain all from FOLLOW(A)
            let last_symbol = res_symbol(&production.rhs[production.rhs.len() - 1]);
            let lhs_follows: Follow = follow_sets[lhs_symbol].iter().copied().collect();
            follow_sets[last_symbol].extend(lhs_follows.iter());
        }
    }
    follow_sets
}

fn closure(state: &mut LRState, grammar: &Grammar, first_sets: &FirstSets) {
    let mut added;
    loop {
        added = false;
        for item in &state.items {
            if let Some(symbol) = item.symbol_at_position(grammar) {
                if grammar.is_nonterm(symbol) {
                    let nonterm = grammar.symbol_to_nonterm(symbol);
                    for prod in &grammar.nonterminals.as_ref().unwrap()[nonterm].productions {
                        let new_follow = Follow::new();
                        // 1. Find first set of substring that follow symbol at position
                        // 2. If this first set can produce EMPTY add follow of current item
                        //
                        // 3. Construct new items from production where LHS is
                        // symbol, position is 0 and follow is calculated set
                        //let new_item = LRItem::new();
                        todo!()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::{
        index::ProdIndex,
        lang::{
            grammar::Grammar,
            parser::GrammarParser,
            table::{first_sets, LRItem},
        },
    };

    use super::follow_sets;

    fn test_grammar() -> Grammar {
        GrammarParser::default().parse(
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

    #[test]
    fn test_first_sets() {
        let grammar = test_grammar();
        let first_sets = first_sets(&grammar);

        assert_eq!(first_sets.len(), 13);

        // First of terminal is just a terminal itself.
        assert_eq!(
            &first_sets[grammar.symbol_index("id")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["id"]).into_iter())
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("F")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["(", "id"]).into_iter())
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("T")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["(", "id"]).into_iter())
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("E")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["(", "id"]).into_iter())
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Ep")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["+", "EMPTY"]).into_iter())
        );
        assert_eq!(
            &first_sets[grammar.symbol_index("Tp")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["*", "EMPTY"]).into_iter())
        );
    }

    #[test]
    fn test_follow_sets() {
        let grammar = test_grammar();
        let follow_sets = follow_sets(&grammar, &first_sets(&grammar));

        assert_eq!(
            &follow_sets[grammar.symbol_index("E")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&[")", "STOP"]).into_iter())
        );
        dbg!(grammar.symbol_names(&follow_sets[grammar.symbol_index("Ep")]));
        assert_eq!(
            &follow_sets[grammar.symbol_index("Ep")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&[")", "STOP"]).into_iter())
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("T")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["+", ")", "STOP"]).into_iter())
        );
        assert_eq!(
            &follow_sets[grammar.symbol_index("Tp")],
            &HashSet::<_>::from_iter(grammar.symbol_indexes(&["+", ")", "STOP"]).into_iter())
        );
    }

    #[test]
    fn test_symbol_at_position() {
        let grammar = test_grammar();

        let prod = ProdIndex(1);
        let mut item = LRItem::new(prod);
        assert_eq!(
            &grammar.symbol_names(&grammar.productions.as_ref().unwrap()[prod].rhs_symbols()),
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
}
