//! Calculating LR tables

use std::{collections::HashSet, iter::Extend};

use indexmap::IndexSet;

use crate::parser::{NonTermIndex, SymbolIndex, SymbolVec};

use super::grammar::{res_symbol, Grammar, ResolvingSymbolIndex};

pub(in crate::lang) struct LRTable {}
pub(in crate::lang) fn calculate_lr_tables(grammar: Grammar) {
    let first_sets = first_sets(&grammar);
    check_empty_sets(&grammar, &first_sets);
    let follow_sets = follow_sets(&grammar, &first_sets);
}

/// Check for states with GOTO links but without SHIFT links.
/// This is invalid as the GOTO link will never be traversed.
fn check_empty_sets(grammar: &Grammar, first_sets: &SymbolVec<IndexSet<SymbolIndex>>) {
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
fn first_sets(grammar: &Grammar) -> SymbolVec<IndexSet<SymbolIndex>> {
    let mut first_sets = SymbolVec::new();

    if let Some(ref terminals) = grammar.terminals {
        for terminal in terminals {
            let mut new_set = IndexSet::new();
            new_set.insert(terminal.idx.to_symbol_index());
            first_sets.push(new_set);
        }
    }
    if let Some(ref nonterminals) = grammar.nonterminals {
        nonterminals
            .iter()
            .for_each(|_| first_sets.push(IndexSet::new()));
    }

    // EMPTY derives EMPTY
    first_sets[grammar.empty_index].insert(grammar.empty_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in grammar.productions.as_ref().unwrap() {
            let lhs_nonterm = grammar.nonterm_to_symbol(production.nonterminal);
            let mut break_out = false;

            for rhs_symbol in production.rhs.iter().map(|assgn| res_symbol!(assgn)) {
                let mut rhs_firsts = first_sets[rhs_symbol].clone();
                let empty = rhs_firsts.remove(&grammar.empty_index);

                let should_add = rhs_firsts
                    .iter()
                    .any(|x| !first_sets[lhs_nonterm].contains(x));

                if should_add {
                    additions = true;
                    first_sets[lhs_nonterm].extend(rhs_firsts);
                }

                // If current RHS symbol can't derive EMPTY this production
                // can't add any more members of the first set for LHS
                // nonterminal.
                if !empty {
                    break_out = true;
                    break;
                }
            }
            if !break_out {
                // If we reached the end of the RHS and each symbol along the
                // way could derive EMPTY than we must add EMPTY to the first
                // set of LHS symbol.
                if !first_sets[lhs_nonterm].contains(&grammar.empty_index) {
                    first_sets[lhs_nonterm].insert(grammar.empty_index);
                    additions = true;
                }
            }
        }
    }
    first_sets
}

/// Calculates the sets of terminals that can follow some non-terminal for the
/// given grammar.
///
/// The dragon book p.221
fn follow_sets(
    grammar: &Grammar,
    first_sets: &SymbolVec<IndexSet<SymbolIndex>>,
) -> SymbolVec<HashSet<SymbolIndex>> {
    let mut follow_sets = SymbolVec::new();
    for _ in 0..first_sets.len() {
        follow_sets.push(HashSet::new());
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
                let rhs_symbol = res_symbol!(assign);
                let elements = follow_sets[rhs_symbol].len();
                let mut break_out = false;
                for rassign in &production.rhs[idx + 1..] {
                    let follow_symbols = &first_sets[res_symbol!(rassign)];

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
                    let lhs_follows: HashSet<SymbolIndex> =
                        follow_sets[lhs_symbol].iter().copied().collect();
                    follow_sets[rhs_symbol].extend(lhs_follows.iter());
                }

                if elements > follow_sets[rhs_symbol].len() {
                    additions = true
                }
            }
            // At the end handle situation A -> α B where FOLLOW(B) should
            // contain all from FOLLOW(A)
            let last_symbol = res_symbol!(production.rhs[production.rhs.len()]);
            let lhs_follows: HashSet<SymbolIndex> =
                follow_sets[lhs_symbol].iter().copied().collect();
            follow_sets[last_symbol].extend(lhs_follows.iter());
        }
    }
    follow_sets
}

#[cfg(test)]
mod tests {
    use indexmap::IndexSet;

    use crate::lang::parser::GrammarParser;

    #[test]
    fn first_sets() {
        let grammar = GrammarParser::default().parse(
            r#"
            S: A B | B C;
            A: EMPTY | B;
            D: A C;
            terminals
            B: "b";
            C: "c";
            "#
            .into(),
        );
        dbg!(super::first_sets(&grammar));
        dbg!(&grammar.nonterminals);
        assert_eq!(super::first_sets(&grammar).len(), 8);

        // First of terminal is just a terminal itself.
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("B")],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B"]).into_iter())
        );

        // First of S is b.
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("S")],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B"]).into_iter())
        );

        // A can derive EMPTY, thus first(C) will be added to first(D)
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("D")],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B", "C"]).into_iter())
        );
        // A can derive EMPTY
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("A")],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B", "EMPTY"]).into_iter())
        );
    }
}
