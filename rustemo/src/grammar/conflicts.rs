//! Analyze and report problems with a grammar.

use std::fmt::Display;

use rustemo_rt::index::{ProdIndex, StateVec, TermIndex};

use crate::table::{Action, LRState};

use super::Grammar;

pub enum ConflictKind {
    ShiftReduce(ProdIndex),
    ReduceReduce(ProdIndex, ProdIndex),
}

pub struct Conflict<'a> {
    state: &'a LRState,
    follow: TermIndex,
    kind: ConflictKind,
}

impl<'a> Display for Conflict<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")?;
        Ok(())
    }
}

pub fn print_conflicts_report(conflicts: &Vec<Conflict>, grammar: &Grammar) {
    for conflict in conflicts {
        println!("In {}", conflict.state.to_string(grammar));
        print!(
            "When I saw {} and see token {} ahead I can't decide",
            grammar.symbol_name(conflict.state.symbol),
            grammar.symbol_name(grammar.term_to_symbol_index(conflict.follow))
        );
        match conflict.kind {
            ConflictKind::ShiftReduce(prod) => {
                println!(
                    " should I shift or reduce by production:\n{}",
                    grammar.productions[prod].to_string(grammar)
                );
            }
            ConflictKind::ReduceReduce(prod1, prod2) => {
                println!(
                    " should I reduce by production:\n{}\nor production:\n{}\n",
                    grammar.productions[prod1].to_string(grammar),
                    grammar.productions[prod2].to_string(grammar)
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
        "{} conflicts. {} Shift/Reduce and {} Reduce/Reduce.",
        shift_reduce_len + &reduce_reduce_len,
        shift_reduce_len,
        reduce_reduce_len
    );
}

pub fn get_conflicts(states: &StateVec<LRState>) -> Vec<Conflict> {
    let mut conflicts = vec![];
    for state in states {
        conflicts.extend(
            state
                .actions
                .iter()
                .enumerate()
                .filter_map(|(term_index, actions)| -> Option<Conflict> {
                    if actions.len() > 1 {
                        // Assumtion is that only two action at most can exist
                        // for some terminal lookahead.
                        assert!(actions.len() == 2);

                        // First figure out the type of conflict.
                        let kind = match &actions[..] {
                            // Shift/Reduce
                            [Action::Shift(..), Action::Reduce(prod, ..)]
                                | [Action::Reduce(prod, ..), Action::Shift(..)]=>
                                    ConflictKind::ShiftReduce(*prod),
                            // Reduce/Reduce
                            [Action::Reduce(prod1, ..), Action::Reduce(prod2, ..)] =>
                                ConflictKind::ReduceReduce(*prod1,  *prod2),
                            _ => unreachable!()
                        };

                        Some(Conflict {
                            state,
                            follow: TermIndex(term_index),
                            kind
                        })
                    } else {
                        None
                    }
                }),
        );
    }
    conflicts
}
