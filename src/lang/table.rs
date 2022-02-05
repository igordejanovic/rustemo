//! Calculating LR tables

use indexmap::IndexSet;

use crate::parser::{SymbolIndex, TermIndex};

use super::grammar::Grammar;

pub(in crate::lang) struct LRTable {}
pub(in crate::lang) fn calculate_lr_tables(grammar: Grammar) {
    let first_sets = first_sets(&grammar);
}

fn first_sets(grammar: &Grammar) -> Vec<IndexSet<SymbolIndex>> {
    let mut first_sets = Vec::new();

    let mut terminals_count = 0;
    if let Some(ref terminals) = grammar.terminals {
        for terminal in terminals {
            let mut new_set = IndexSet::new();
            new_set.insert(terminal.idx.to_symbol_index());
            first_sets.push(new_set);
        }
        terminals_count = terminals.len();
    }
    if let Some(ref nonterminals) = grammar.nonterminals {
        for nonterminal in nonterminals {
            let mut new_set = IndexSet::new();
            new_set.insert(nonterminal.idx.to_symbol_index(terminals_count));
            first_sets.push(new_set);
        }
    }

    first_sets
}

#[cfg(test)]
mod tests {
    use indexmap::IndexSet;

    use crate::lang::parser::GrammarParser;

    #[test]
    fn first_sets() {
        let grammar = GrammarParser::default().parse(
            r#"
            S: A B;
            A: EMPTY | B;
            B: "b";
            "#
            .into(),
        );
        assert_eq!(super::first_sets(&grammar).len(), 5);

        // First of S is S, A, B and b.
        assert_eq!(
            super::first_sets(&grammar)[grammar.symbol_index("S").0],
            IndexSet::<_>::from_iter(grammar.symbol_indexes(&["S", "A", "B", "b"]).into_iter())
        );
    }
}
