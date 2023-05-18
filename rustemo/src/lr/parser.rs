use crate::debug::log;
use crate::error::Result;
use crate::lexer::{Context, Input, Lexer, Token, TokenRecognizer};
use crate::location::{Location, Position};
use crate::parser::Parser;
use crate::{err, Error};
#[cfg(debug_assertions)]
use colored::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

use super::builder::LRBuilder;

/// Provides LR actions and GOTOs given the state and term/nonterm.
pub trait ParserDefinition<TR: TokenRecognizer, S, P, T, NT> {
    fn action(&self, state: S, term_index: T) -> Action<S, P>;
    fn goto(&self, state: S, nonterm: NT) -> S;
    fn recognizers(&self, state: S) -> Vec<&TR>;
}

#[derive(Debug, Copy, Clone)]
pub enum Action<S, P> {
    Shift(S),
    Reduce(P, usize),
    Accept,
    Error,
}

struct StackItem<S> {
    state: S,
    range: Range<usize>,
    location: Location,
}

impl<S: Debug> Debug for StackItem<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State({:?}, {:?} {:?})",
            self.state, self.range, self.location
        )
    }
}

#[derive(Debug)]
pub struct LRParser<
    S,
    P,
    T,
    NT,
    D: ParserDefinition<TR, S, P, T, NT> + 'static,
    TR: TokenRecognizer,
> {
    definition: &'static D,
    parse_stack: Vec<StackItem<S>>,
    partial_parse: bool,
    phantom: PhantomData<(TR, P, T, NT)>,
}

impl<S, P, T, NT, D, TR> LRParser<S, P, T, NT, D, TR>
where
    S: Copy,
    T: Default + Debug + PartialEq,
    D: ParserDefinition<TR, S, P, T, NT>,
    TR: TokenRecognizer<TokenKind = T>,
{
    pub fn new(definition: &'static D, state: S, partial_parse: bool) -> Self {
        Self {
            definition,
            parse_stack: vec![StackItem {
                state,
                range: 0..0,
                location: Location {
                    start: Position::Position(0),
                    end: None,
                },
            }],
            partial_parse,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn push_state<I: Input + ?Sized>(
        &mut self,
        context: &mut Context<I>,
        state: S,
    ) {
        self.parse_stack.push(StackItem {
            state,
            range: context.range.start..context.range.end,
            location: context.location,
        });
    }

    #[inline]
    fn pop_states<I>(
        &mut self,
        context: &mut Context<I>,
        states: usize,
    ) -> (S, Range<usize>, Location)
    where
        I: Input<Output = I> + ?Sized,
    {
        let states_removed =
            self.parse_stack.split_off(self.parse_stack.len() - states);
        let state = self.parse_stack.last().unwrap().state;

        let (range, location) = if states == 0 {
            // EMPTY reduction
            (
                context.position..context.position,
                Location {
                    start: context.location.start,
                    end: Some(context.location.start),
                },
            )
        } else {
            (
                states_removed[0].range.start
                    ..states_removed.last().unwrap().range.end,
                Location {
                    start: states_removed[0].location.start,
                    end: states_removed.last().unwrap().location.end,
                },
            )
        };
        (state, range, location)
    }

    fn next_token<'i, I, L>(
        &self,
        lexer: &L,
        context: &mut Context<'i, I>,
        state: S,
    ) -> Result<Token<'i, I, T>>
    where
        L: Lexer<I, TR>,
        I: Input<Output = I> + ?Sized,
    {
        let expected_recognizers = self.definition.recognizers(state);
        let stop_kind = <T as Default>::default();
        lexer
            .next_token(context, &expected_recognizers)
            .or_else(|| {
                if self.partial_parse
                    && expected_recognizers
                        .iter()
                        .any(|tr| tr.token_kind() == stop_kind)
                {
                    Some(Token {
                        kind: stop_kind,
                        value: &context.input
                            [context.position..context.position],
                        location: context.location,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                let expected = expected_recognizers
                    .iter()
                    .map(|recognizer| format!("{:?}", recognizer.token_kind()))
                    .collect::<Vec<_>>();
                let expected = if expected.len() > 1 {
                    format!("one of {}", expected.join(", "))
                } else {
                    expected[0].to_owned()
                };
                Error::Error {
                    message: format!(
                        "...{}...\nExpected {}.",
                        context.input.context_str(context.position),
                        expected
                    ),
                    file: Some(context.file.clone()),
                    location: Some(Location::from(context)),
                }
            })
    }
}

impl<'i, S, P, T, NT, I, D, L, B, TR> Parser<'i, I, L, B, TR>
    for LRParser<S, P, T, NT, D, TR>
where
    S: Debug + Copy,
    T: Default + PartialEq + Debug + Copy,
    P: Debug + Copy + Into<NT>,
    I: Debug + Input<Output = I> + ?Sized,
    D: ParserDefinition<TR, S, P, T, NT>,
    L: Lexer<I, TR>,
    TR: TokenRecognizer<TokenKind = T>,
    B: LRBuilder<'i, I, P, T>,
{
    fn parse(
        &mut self,
        context: &mut Context<'i, I>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output> {
        log!(
            "{} at {}{:?}: '{}'",
            "Context".green(),
            context.position,
            context.location,
            context.input.context_str(context.position)
        );

        let mut state = self.parse_stack.last().unwrap().state;

        log!("{}: {:#?}", "Stack".green(), self.parse_stack);
        log!("{}: {:?}", "Current state".green(), state);

        let mut next_token = self.next_token(lexer, context, state)?;
        log!("{}: {:?}", "Token ahead".green(), next_token);

        loop {
            let action = self.definition.action(state, next_token.kind);

            match action {
                Action::Shift(state_id) => {
                    state = state_id;
                    context.range = context.position..(context.position + next_token.value.len());
                    let new_location = next_token.value.location_after(context.location);
                    context.location.end = Some(new_location.start);
                    context.layout = context.layout_ahead;

                    log!("{} to state {:?} at location {:?} with token {:?}",
                        "Shifting".bold().green(),
                        state_id,
                        context.location,
                        next_token
                    );
                    self.push_state(context, state);
                    builder.shift_action(context, next_token);

                    context.position = context.range.end;
                    context.location = new_location;
                    log!(
                        "{} at {}{:?}:\n{}\n",
                        "Context".green(),
                        context.position,
                        context.location,
                        context.input.context_str(context.position)
                    );
                    next_token = self.next_token(lexer, context, state)?;
                    log!("{}: {:?}", "Token ahead".green(), next_token);
                }
                Action::Reduce(prod, prod_len) => {
                    log!(
                        "{} by production '{:?}', size {:?}",
                        "Reduce".bold().green(),
                        prod,
                        prod_len
                    );
                    let (from_state, range, location) =
                        self.pop_states(context, prod_len);
                    context.range = range;
                    state = self.definition.goto(from_state, prod.into());
                    let context_location = context.location;
                    context.location = location;
                    self.push_state(context, state);
                    log!("{} {:?} -> {:?}", "GOTO".green(), from_state, state);
                    builder.reduce_action(context, prod, prod_len);
                    context.location = context_location;
                }
                Action::Accept => break,
                // This can't happen for context-aware lexing. If there is no
                // action for a lookahead then the lookahead would not be found.
                // The only place where this can trigger is when parsing layout.
                // It may happen that a wrong recognition is done in the content
                // after a layout. Also, in the future, if parser composition
                // would be done similar problem may arise.
                Action::Error => err!(format!("Can't continue in state {state:?} with lookahead {next_token:?}."))?,
            }
            log!("{}: {:#?}", "Stack".green(), self.parse_stack);
            log!("{}: {:?}", "Current state".green(), state);
        }
        Ok(builder.get_result())
    }
}

mod tests {}
