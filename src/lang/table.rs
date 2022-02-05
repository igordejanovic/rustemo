//! Calculating LR tables

use std::collections::HashSet;

use indexmap::IndexSet;

use crate::parser::SymbolIndex;

use super::grammar::{res, Grammar, ResolvingSymbolIndex};

pub(in crate::lang) struct LRTable {}
pub(in crate::lang) fn calculate_lr_tables(grammar: Grammar) {
    let first_sets = first_sets(&grammar);
}

// Calculates the sets of terminals that can start the sentence derived from all
// grammar symbols. The Dragon book p. 221.
fn first_sets(grammar: &Grammar) -> Vec<IndexSet<SymbolIndex>> {
    let mut first_sets = Vec::new();

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
    first_sets[grammar.empty_index.0].insert(grammar.empty_index);

    let mut additions = true;
    while additions {
        additions = false;
        for production in grammar.productions.as_ref().unwrap() {
            let nonterm_idx = grammar.nonterm_to_symbol(production.nonterminal).0;
            let mut break_out = false;

            for rhs_symbol in production.rhs.iter().map(|assgn| res!(assgn.symbol)) {
                let mut rhs_firsts = first_sets[rhs_symbol.0].clone();
                let empty = rhs_firsts.remove(&grammar.empty_index);

                let should_add = rhs_firsts
                    .iter()
                    .any(|x| !first_sets[nonterm_idx].contains(x));

                if should_add {
                    additions = true;
                    let f = first_sets[rhs_symbol.0].clone();
                    first_sets[nonterm_idx].extend(f);
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
                if !first_sets[nonterm_idx].contains(&grammar.empty_index) {
                    first_sets[nonterm_idx].insert(grammar.empty_index);
                    additions = true;
                }
            }
        }
    }
    // Remove EMPTY from all sets
    first_sets
        .into_iter()
        .map(|mut set| {
            set.remove(&grammar.empty_index);
            set
        })
        .collect()
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
        assert_eq!(super::first_sets(&grammar).len(), 6);

        // First of terminal is just a terminal itself.
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("B").0],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B"]).into_iter())
        );

        // First of S is b.
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("S").0],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B"]).into_iter())
        );

        // A can derive EMPTY, thus first(C) will be added to first(D)
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("D").0],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B", "C"]).into_iter())
        );
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("A").0],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["B"]).into_iter())
        );
    }
}
