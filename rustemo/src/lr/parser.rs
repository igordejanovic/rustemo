use crate::debug::log;
use crate::error::Result;
use crate::index::{NonTermIndex, ProdIndex, StateIndex, TermIndex};
use crate::lexer::{Context, Input, Lexer};
use crate::parser::Parser;
use crate::Error;
use std::fmt::{Debug, Display};

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
    Shift(StateIndex),
    Reduce(ProdIndex, usize, NonTermIndex),
    Accept,
    Error,
}

#[derive(Debug)]
struct StackItem {
    state: StateIndex,
    start_pos: usize,
    end_pos: usize,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Shift(state) => {
                write!(f, "Shift(StateIndex({state}))")
            }
            Action::Reduce(prod, len, nonterm) => {
                write!(
                    f,
                    "Reduce(ProdIndex({}), {}, NonTermIndex({}))",
                    prod, len, nonterm
                )
            }
            Action::Accept => write!(f, "Accept"),
            Action::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug)]
pub struct LRParser<D: ParserDefinition + 'static> {
    definition: &'static D,
    parse_stack: Vec<StackItem>,
}

impl<D: ParserDefinition> LRParser<D> {
    pub fn new(definition: &'static D, state: StateIndex) -> Self {
        Self {
            definition,
            parse_stack: vec![StackItem {
                state,
                start_pos: 0,
                end_pos: 0,
            }],
        }
    }

    #[inline]
    fn push_state<I: Input + ?Sized, LO>(
        &mut self,
        context: &mut Context<I, LO, StateIndex>,
        state: StateIndex,
    ) {
        self.parse_stack.push(StackItem {
            state,
            start_pos: context.start_pos,
            end_pos: context.end_pos,
        });
        context.state = state;
    }

    #[inline]
    fn pop_states<I: Input + ?Sized, LO>(
        &mut self,
        context: &mut Context<I, LO, StateIndex>,
        states: usize,
    ) -> (StateIndex, usize, usize) {
        let states_removed =
            self.parse_stack.split_off(self.parse_stack.len() - states);
        context.state = self.parse_stack.last().unwrap().state;

        let start_pos;
        let end_pos;
        if states == 0 {
            // EMPTY reduction
            start_pos = context.position;
            end_pos = context.position;
        } else {
            start_pos = states_removed[0].start_pos;
            end_pos = states_removed.last().unwrap().end_pos;
        }
        (context.state, start_pos, end_pos)
    }
}

impl<'i, I, D, L, B, LO, TK> Parser<'i, I, L, B, LO, StateIndex, TK>
    for LRParser<D>
where
    I: Debug + Input + ?Sized,
    D: ParserDefinition,
    L: Lexer<'i, I, LO, StateIndex, TK>,
    B: LRBuilder<'i, I, LO, TK>,
    TK: Debug + Into<TermIndex> + Copy,
{
    fn parse(
        &mut self,
        context: &mut Context<'i, I, LO, StateIndex>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output> {
        log!(
            "Position={}: {}",
            context.position,
            context.input.context_str(context.position)
        );
        context.state = self.parse_stack.last().unwrap().state;
        let mut next_token = lexer.next_token(context)?;
        loop {
            let current_state = self.parse_stack.last().unwrap().state;
            log!("Stack: {:?}", self.parse_stack);
            log!("Current state: {:?}", current_state);
            log!("Token ahead: {:?}", next_token);

            let action = self
                .definition
                .action(current_state, next_token.kind.into());

            log!("Action: {:?}", action);

            match action {
                Action::Shift(state_id) => {
                    log!(
                        "Shifting to state {:?} with token {:?}",
                        state_id,
                        next_token
                    );
                    context.start_pos = context.position;
                    context.end_pos = context.position + next_token.value.len();
                    self.push_state(context, state_id);

                    let new_location = next_token.value.new_location(context.location);
                    builder.shift_action(context, next_token);

                    context.position = context.end_pos;
                    log!(
                        "Position={}: {}",
                        context.position,
                        context.input.context_str(context.position)
                    );
                    context.location = Some(new_location);
                    next_token = lexer.next_token(context)?;
                }
                Action::Reduce(prod_idx, prod_len, nonterm_id) => {
                    log!(
                        "Reduce by production '{}', size {:?}, non-terminal {:?}",
                        prod_idx,
                        prod_len,
                        nonterm_id
                    );
                    let (from_state, start_pos, end_pos) =
                        self.pop_states(context, prod_len);
                    context.start_pos = start_pos;
                    context.end_pos = end_pos;
                    let to_state = self.definition.goto(from_state, nonterm_id);
                    self.push_state(context, to_state);
                    log!("GOTO {:?} -> {:?}", from_state, to_state);
                    builder.reduce_action(context, prod_idx, prod_len);
                }
                Action::Accept => break,
                // This can't happen for context-aware lexing. If there is no
                // action for a lookahead then the lookahead would not be found.
                // The only place where this can trigger is when parsing layout.
                // It may happen that a wrong recognition is done in the content
                // after a layout. Also, in the future, if parser composition
                // would be done similar problem may arise.
                Action::Error => Err(
                    Error::Error(
                        format!("Can't continue in state {current_state} with lookahead {next_token:?}.")))?,
            }
        }
        Ok(builder.get_result())
    }
}

mod tests {}
