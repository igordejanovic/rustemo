use crate::debug::log;
use crate::error::Result;
use crate::index::TermIndex;
use crate::lexer::{AsStr, Context, Input, Lexer, Token, TokenRecognizer};
use crate::location::Location;
use crate::parser::Parser;
use crate::{err, Error};
use colored::*;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::Range;

use super::builder::LRBuilder;

/// Provides LR actions and GOTOs given the state and term/nonterm.
pub trait ParserDefinition<TR: TokenRecognizer, S, P, NT> {
    fn action(&self, state: S, term_index: TermIndex) -> Action<S, P>;
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

impl<S, P> Action<S, P>
where
    S: Debug,
    P: Debug,
{
    pub fn generate(&self) -> String {
        match self {
            Action::Shift(state) => format!("Shift({:?})", state),
            Action::Reduce(prod, len) => {
                format!("Reduce({:?}, {:?})", prod, len)
            }
            Action::Accept => String::from("Accept"),
            Action::Error => String::from("Error"),
        }
    }
}

#[derive(Debug)]
struct StackItem<S> {
    state: S,
    range: Range<usize>,
}

impl<S, P> Display for Action<S, P>
where
    S: Display,
    P: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Shift(state) => {
                write!(f, "Shift to {}", state)
            }
            Action::Reduce(prod, _) => {
                write!(f, "Reduce by {}", prod)
            }
            Action::Accept => write!(f, "Accept"),
            Action::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug)]
pub struct LRParser<
    S,
    P,
    NT,
    D: ParserDefinition<TR, S, P, NT> + 'static,
    TR: TokenRecognizer,
> {
    definition: &'static D,
    parse_stack: Vec<StackItem<S>>,
    partial_parse: bool,
    phantom: PhantomData<(TR, P, NT)>,
}

impl<
        S: Copy,
        P,
        NT,
        D: ParserDefinition<TR, S, P, NT>,
        TR: TokenRecognizer,
    > LRParser<S, P, NT, D, TR>
{
    pub fn new(definition: &'static D, state: S, partial_parse: bool) -> Self {
        Self {
            definition,
            parse_stack: vec![StackItem { state, range: 0..0 }],
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
        });
    }

    #[inline]
    fn pop_states<I: Input + ?Sized>(
        &mut self,
        context: &mut Context<I>,
        states: usize,
    ) -> (S, Range<usize>) {
        let states_removed =
            self.parse_stack.split_off(self.parse_stack.len() - states);
        let state = self.parse_stack.last().unwrap().state;

        let range = if states == 0 {
            // EMPTY reduction
            context.position..context.position
        } else {
            states_removed[0].range.start
                ..states_removed.last().unwrap().range.end
        };
        (state, range)
    }

    fn next_token<'i, I, L>(
        &self,
        lexer: &L,
        context: &mut Context<'i, I>,
        state: S,
    ) -> Result<Token<'i, I, <TR as TokenRecognizer>::TokenKind>>
    where
        L: Lexer<I, TR>,
        I: Input + ?Sized,
    {
        let expected_recognizers = self.definition.recognizers(state);
        let stop_kind = <TR as TokenRecognizer>::TokenKind::default();
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
                        value: context
                            .input
                            .slice(&(context.position..context.position)),
                        location: context.location,
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                let expected = expected_recognizers
                    .iter()
                    .map(|recognizer| recognizer.token_kind().as_str())
                    .collect::<Vec<_>>();
                let expected = if expected.len() > 1 {
                    format!("one of {}", expected.join(", "))
                } else {
                    expected[0].into()
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

impl<'i, S, P, NT, I, D, L, B, TR> Parser<'i, I, L, B, TR>
    for LRParser<S, P, NT, D, TR>
where
    S: Debug + Copy,
    P: Debug + Copy + Into<NT>,
    I: Debug + Input + ?Sized,
    D: ParserDefinition<TR, S, P, NT>,
    L: Lexer<I, TR>,
    TR: TokenRecognizer,
    B: LRBuilder<'i, I, P, <TR as TokenRecognizer>::TokenKind>,
{
    fn parse(
        &mut self,
        context: &mut Context<'i, I>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output> {
        log!(
            "Position={}: {}",
            context.position,
            context.input.context_str(context.position)
        );

        let mut state = self.parse_stack.last().unwrap().state;

        let mut next_token = self.next_token(lexer, context, state)?;
        loop {
            log!("Stack: {:?}", self.parse_stack);
            log!("Current state: {:?}", state);
            log!("Token ahead: {:?}", next_token);

            let action = self.definition.action(state, next_token.kind.into());

            log!("Action: {:?}", action);

            match action {
                Action::Shift(state_id) => {
                    log!("{} to state {:?} with token {:?}",
                        "Shifting".bold().green(),
                        state_id,
                        next_token
                    );
                    state = state_id;
                    context.range = context.position..(context.position + next_token.value.len());
                    self.push_state(context, state);

                    let new_location = next_token.value.location_after(context.location);
                    context.layout = context.layout_ahead;
                    builder.shift_action(context, next_token);

                    context.position = context.range.end;
                    log!(
                        "Position={}: {}",
                        context.position,
                        context.input.context_str(context.position)
                    );
                    context.location = new_location;
                    next_token = self.next_token(lexer, context, state)?;
                }
                Action::Reduce(prod, prod_len) => {
                    log!(
                        "{} by production '{:?}', size {:?}",
                        "Reduce".bold().green(),
                        prod,
                        prod_len
                    );
                    let (from_state, range) =
                        self.pop_states(context, prod_len);
                    context.range = range;
                    state = self.definition.goto(from_state, prod.into());
                    self.push_state(context, state);
                    log!("GOTO {:?} -> {:?}", from_state, state);
                    builder.reduce_action(context, prod, prod_len);
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
        }
        Ok(builder.get_result())
    }
}

mod tests {}
