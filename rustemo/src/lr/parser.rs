use crate::context::Context;
use crate::debug::log;
use crate::error::{error_expected, Result};
use crate::input::Input;
use crate::lexer::{Lexer, Token};
use crate::location::Location;
use crate::lr::builder::SliceBuilder;
use crate::parser::{Parser, State};
use crate::{err, Error};
#[cfg(debug_assertions)]
use colored::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;

use super::builder::LRBuilder;

/// Provides LR actions and GOTOs given the state and term/nonterm.
pub trait ParserDefinition<S, P, TK, NTK> {
    fn actions(&self, state: S, token: TK) -> Vec<Action<S, P>>;
    fn goto(&self, state: S, nonterm: NTK) -> S;
    fn expected_token_kinds(&self, state: S) -> Vec<TK>;
}

/// An action executed by the (G)LR Parser during parsing
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

struct ParseStack<S, I: ?Sized, C, TK> {
    stack: Vec<StackItem<S>>,
    phantom: PhantomData<(C, TK, I)>,
}

impl<S: Debug, I: ?Sized, C, TK> Debug for ParseStack<S, I, C, TK> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseStack")
            .field("stack", &self.stack)
            .finish()
    }
}

impl<'i, I, C, S, TK> ParseStack<S, I, C, TK>
where
    I: Input + ?Sized,
    C: Context<'i, I, S, TK>,
    S: State,
{
    fn new(context: &mut C, start_state: S) -> ParseStack<S, I, C, TK> {
        Self {
            stack: vec![StackItem {
                state: start_state,
                range: context.range(),
                location: context.location(),
            }],
            phantom: PhantomData,
        }
    }

    #[inline]
    fn state(&self) -> S {
        self.stack.last().unwrap().state
    }

    #[inline]
    fn push_state(&mut self, context: &mut C, state: S) {
        self.stack.push(StackItem {
            state,
            range: context.range(),
            location: context.location(),
        });
        context.set_state(state);
    }

    fn pop_states(
        &mut self,
        context: &mut C,
        states: usize,
    ) -> (S, Range<usize>, Location) {
        let states_removed = self.stack.split_off(self.stack.len() - states);
        let state = self.stack.last().unwrap().state;

        let (range, location) = if states == 0 {
            // EMPTY reduction
            (
                context.position()..context.position(),
                Location {
                    start: context.location().start,
                    end: Some(context.location().start),
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
}

/// An implementation of LR parsing
pub struct LRParser<
    'i,
    C: Context<'i, I, S, TK>,
    S: State,
    P,
    TK: Default,
    NTK,
    D: ParserDefinition<S, P, TK, NTK>,
    L: Lexer<'i, C, S, TK, Input = I>,
    B,
    I: Input + ?Sized,
> {
    definition: &'i D,
    file_name: String,
    content: Option<<<L as Lexer<'i, C, S, TK>>::Input as ToOwned>::Owned>,
    partial_parse: bool,
    start_position: usize,
    start_state: S,
    has_layout: bool,
    lexer: Rc<L>,
    builder: RefCell<B>,
    phantom: PhantomData<(P, NTK, I)>,
}

type LayoutParser<'i, C, S, P, TK, NTK, D, L, I> =
    Option<LRParser<'i, C, S, P, TK, NTK, D, L, SliceBuilder<'i, I>, I>>;

impl<'i, C, S, P, I, TK, NTK, D, L, B>
    LRParser<'i, C, S, P, TK, NTK, D, L, B, I>
where
    C: Context<'i, I, S, TK>,
    S: State,
    I: Input + ?Sized,
    TK: Default,
    D: ParserDefinition<S, P, TK, NTK>,
    L: Lexer<'i, C, S, TK, Input = I>,
    B: LRBuilder<'i, I, C, S, P, TK>,
{
    pub fn new(
        definition: &'i D,
        state: S,
        partial_parse: bool,
        has_layout: bool,
        lexer: L,
        builder: B,
    ) -> Self {
        Self::new_default(
            definition,
            state,
            partial_parse,
            has_layout,
            Rc::new(lexer),
            RefCell::new(builder),
        )
    }

    pub(crate) fn new_default(
        definition: &'i D,
        state: S,
        partial_parse: bool,
        has_layout: bool,
        lexer: Rc<L>,
        builder: RefCell<B>,
    ) -> Self {
        Self {
            definition,
            partial_parse,
            file_name: "<str>".into(),
            content: None,
            start_position: 0,
            start_state: state,
            has_layout,
            lexer,
            builder,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn location_str(&self, file: &str, location: Location) -> String {
        format!("{}:{:?}", file.to_owned(), location)
    }

    fn next_token(
        &self,
        input: &'i I,
        context: &mut C,
        layout_parser: &LayoutParser<'i, C, S, P, TK, NTK, D, L, I>,
    ) -> Result<Token<'i, I, TK>>
    where
        // Needed for calling parse_with_context
        P: Debug + Into<NTK> + Copy,
        S: Debug,
        I: Debug,
        TK: Debug + Copy + PartialEq + 'i,
        C: Default,
    {
        // Get next tokens (lexer should skip ws if configured to do so).
        // If error run layout_parser. If there is layout try next tokens again.
        // If no next token can be returned report error returned from the lexer.
        // TODO: Handle lexical ambiguity
        loop {
            let expected_tokens =
                self.definition.expected_token_kinds(context.state());
            if let Some(next_token) = self
                .lexer
                .next_tokens(context, input, expected_tokens)
                .next()
            {
                return Ok(next_token);
            } else {
                // No token found at current position. Try layout if configured.
                if let Some(layout_parser) = layout_parser {
                    log!("\n{}", "*** Parsing layout".red().bold());
                    let current_state = context.state();
                    context.set_state(S::default_layout().unwrap());
                    let p = layout_parser.parse_with_context(context, input);
                    log!("Layout is {p:?}");
                    context.set_state(current_state);
                    if let Ok(Some(layout)) = p {
                        if layout.len() > 0 {
                            log!("Skipping layout: {layout:?}");
                            context.set_layout_ahead(Some(layout));
                            log!("\n{}", "*** Parsing content".red().bold());
                            continue;
                        }
                    }
                }
                // At this point we can't recognize any new token at the current position.
                // This can be Ok if partial parse is configured and STOP is expected.
                // Otherwise we should report error with expected tokens at this position.
                let stop_kind = <TK as Default>::default();
                let expected =
                    self.definition.expected_token_kinds(context.state());
                if self.partial_parse
                    && expected.iter().any(|&t| t == stop_kind)
                {
                    return Ok(Token {
                        kind: stop_kind,
                        value: &input[context.position()..context.position()],
                        location: context.location(),
                    });
                } else {
                    return Err(error_expected(
                        input,
                        &self.file_name,
                        context,
                        &expected,
                    ));
                }
            }
        }
    }
}

impl<'i, C, S, P, I, TK, NTK, D, L, B> Parser<'i, I, C, S, TK>
    for LRParser<'i, C, S, P, TK, NTK, D, L, B, I>
where
    C: Context<'i, I, S, TK> + Default,
    S: State + Debug,
    P: Debug + Copy + Into<NTK>,
    I: Input + ?Sized + Debug + 'i,
    TK: Debug + Copy + Default + PartialEq + 'i,
    D: ParserDefinition<S, P, TK, NTK>,
    L: Lexer<'i, C, S, TK, Input = I>,
    B: LRBuilder<'i, I, C, S, P, TK>,
{
    type Output = B::Output;

    fn parse(&self, input: &'i I) -> Result<Self::Output> {
        log!("\n{}", "*** Parsing started".red().bold());
        log!("\nfile: {}", self.file_name);
        let mut context = C::default();
        context.set_position(self.start_position);
        self.parse_with_context(&mut context, input)
    }

    fn parse_with_context(
        &self,
        context: &mut C,
        input: &'i I,
    ) -> Result<Self::Output> {
        let mut parse_stack: ParseStack<S, I, C, TK> =
            ParseStack::new(context, self.start_state);

        let mut builder = self.builder.borrow_mut();

        // Layout parser is the sajme as Self except it uses SliceBulder to
        // produce the output and it never uses partial parse.
        let layout_parser: LayoutParser<'i, C, S, P, TK, NTK, D, L, I> =
            self.has_layout.then(|| {
                LRParser::new_default(
                    self.definition,
                    S::default_layout().expect("Layout state not defined."),
                    true,
                    false,
                    Rc::clone(&self.lexer),
                    RefCell::new(SliceBuilder::new(input)),
                )
            });

        log!(
            "{} at {}{:?}: '{}'",
            "Context".green(),
            context.position(),
            context.location(),
            input.context_str(context.position())
        );

        let mut state = parse_stack.state();

        log!("{}: {:#?}", "Stack".green(), parse_stack);
        log!("{}: {:?}", "Current state".green(), state);

        let mut next_token = self.next_token(input, context, &layout_parser)?;
        log!("{}: {:?}", "Token ahead".green(), &next_token);

        loop {
            let action = self.definition.actions(state, next_token.kind)[0];

            match action {
                Action::Shift(state_id) => {
                    state = state_id;
                    context.set_range(context.position()..(context.position() + next_token.value.len()));
                    let new_location = next_token.value.location_after(context.location());
                    context.set_location(Location{
                        start: context.location().start,
                        end: Some(new_location.start),
                    });

                    log!("{} to state {:?} at location {:?} with token {:?}",
                        "Shifting".bold().green(),
                        state_id,
                        context.location(),
                        &next_token
                    );
                    parse_stack.push_state(context, state);
                    builder.shift_action(context, next_token);

                    context.set_position(context.range().end);
                    context.set_location(new_location);
                    log!(
                        "{} at {}{:?}:\n{}\n",
                        "Context".green(),
                        context.position(),
                        context.location(),
                        input.context_str(context.position())
                    );
                    next_token = self.next_token(input, context, &layout_parser)?;
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
                        parse_stack.pop_states(context, prod_len);
                    context.set_range(range);
                    state = self.definition.goto(from_state, prod.into());
                    let context_location = context.location();
                    context.set_location(location);
                    parse_stack.push_state(context, state);
                    log!("{} {:?} -> {:?}", "GOTO".green(), from_state, state);
                    builder.reduce_action(context, prod, prod_len);
                    context.set_location(context_location);
                }
                Action::Accept => {
                    log!("{}", "Accept".green().bold());
                    break
                },
                // This can't happen for context-aware lexing. If there is no
                // action for a lookahead then the lookahead would not be found.
                // The only place where this can trigger is when parsing layout.
                // It may happen that a wrong recognition is done in the content
                // after the layout. Also, in the future, if parser composition
                // would be done similar problem may arise.
                Action::Error => err!(format!("Can't continue in state {state:?} with lookahead {next_token:?}."))?,
            }
            log!("{}: {:#?}", "Stack".green(), parse_stack);
            log!("{}: {:?}", "Current state".green(), state);
        }
        Ok(builder.get_result())
    }

    fn parse_file<'a, F: AsRef<Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.content = Some(I::read_file(file.as_ref())?);
        self.file_name = file.as_ref().to_string_lossy().into();
        let parsed = self.parse(self.content.as_ref().unwrap().borrow());
        parsed
    }
}
