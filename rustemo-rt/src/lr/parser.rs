use std::fmt::Display;
use std::fmt::Debug;
use crate::debug::log;
use crate::error::RustemoResult;
use crate::index::{NonTermIndex, ProdIndex, StateIndex, TermIndex};
use crate::lexer::{Context, Lexer, Token};
use crate::parser::Parser;
use crate::lr::lexer::LRContext;

use super::builder::LRBuilder;

/// Provides LR actions and GOTOs given the state and term/nonterm.
pub trait ParserDefinition {
    fn action(&self, state: StateIndex, term_index: TermIndex) -> Action;
    fn goto(
        &self,
        state: StateIndex,
        nonterm_index: NonTermIndex,
    ) -> StateIndex;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Shift(StateIndex, TermIndex),
    Reduce(ProdIndex, usize, NonTermIndex, &'static str),
    Accept,
    Error,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Shift(state, term) => {
                write!(f, "Shift(StateIndex({}), TermIndex({}))", state, term)
            }
            Action::Reduce(prod, len, nonterm, prod_desc) => {
                write!(
                    f,
                    "Reduce(ProdIndex({}), {}, NonTermIndex({}), \"{}\")",
                    prod, len, nonterm, prod_desc
                )
            }
            Action::Accept => write!(f, "Accept"),
            Action::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug)]
pub struct LRParser<D: ParserDefinition + 'static> {
    pub definition: &'static D,
    pub parse_stack: Vec<StateIndex>,
}

impl<D: ParserDefinition> LRParser<D> {
    pub fn new(definition: &'static D) -> Self {
        Self {
            definition,
            parse_stack: vec![StateIndex(0)],
        }
    }

    #[inline]
    fn next_token<C, L, I>(&self, context: &mut C, lexer: &L) -> Token<I>
    where
        C: Context<I>,
        L: Lexer<I, C>,
        I: Debug,
    {
        match lexer.next_token(context) {
            Some(t) => t,
            None => {
                panic!("Error at {}", context.location_str());
            }
        }
    }

    #[inline]
    fn to_state<I>(&mut self, context: &mut LRContext<I>, state: StateIndex) {
        self.parse_stack.push(state);
        context.set_state(state);
    }

    #[inline]
    fn pop_states<I>(&mut self, context: &mut LRContext<I>, states: usize) -> StateIndex {
        let _ = self.parse_stack.split_off(self.parse_stack.len() - states);
        context.set_state(*self.parse_stack.last().unwrap());
        context.state()
    }
}

impl<I, D, L, B> Parser<I, LRContext<I>, L, B> for LRParser<D>
where
    I: Debug,
    D: ParserDefinition,
    L: Lexer<I, LRContext<I>>,
    B: LRBuilder<I>,
{
    fn parse(&mut self, mut context: LRContext<I>, lexer: L, mut builder: B) -> RustemoResult<B::Output> {
        use Action::*;
        let mut next_token = self.next_token(&mut context, &lexer);
        loop {
            let current_state = self.parse_stack.last().unwrap();
            log!("Stack: {:?}", self.parse_stack);
            log!("Current state: {:?}", current_state);
            log!("Token ahead: {:?}", next_token);

            let action =
                self.definition.action(*current_state, next_token.index());

            log!("Action: {:?}", action);

            match action {
                Shift(state_id, term_idx) => {
                    log!(
                        "Shifting to state {:?} with token {:?}",
                        state_id,
                        next_token
                    );
                    self.to_state(&mut context, state_id);
                    builder.shift_action(term_idx, next_token);
                    next_token = self.next_token(&mut context, &lexer);
                }
                Reduce(prod_idx, prod_len, nonterm_id, prod_str) => {
                    log!(
                        "Reduce by production '{:?}', size {:?}, non-terminal {:?}",
                        prod_str,
                        prod_len,
                        nonterm_id
                    );
                    let from_state = self.pop_states(&mut context, prod_len);
                    let to_state = self.definition.goto(from_state, nonterm_id);
                    self.to_state(&mut context, to_state);
                    log!("GOTO {:?} -> {:?}", from_state, to_state);
                    builder.reduce_action(prod_idx, prod_len, prod_str);
                }
                Accept => break,
                Error => panic!("Error!"),
            }
        }
        builder.get_result()
    }
}

mod tests {}
