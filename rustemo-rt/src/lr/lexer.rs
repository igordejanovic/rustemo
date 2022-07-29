use std::cmp::min;

use crate::debug::log;
use crate::error::{RustemoError, RustemoResult};
use crate::grammar::TerminalInfo;
use crate::index::StateIndex;
use crate::lexer::{Context, Lexer, Token};
use crate::location::{Location, Position, LineBased};

#[derive(Debug)]
pub struct LRContext<I> {
    file: String,
    input: I,
    position: usize,
    location: Option<Location>,
    token: Option<Token<I>>,
    layout: Option<I>,
    state: StateIndex,
}

impl<I> LRContext<I> {
    pub fn new(file: String, input: I) -> Self {
        Self {
            file,
            input,
            position: 0,
            token: None,
            location: None,
            layout: None,
            state: StateIndex(0),
        }
    }
}

impl<I> Context<I> for LRContext<I> {
    #[inline]
    fn file(&self) -> String {
        self.file.clone()
    }

    #[inline]
    fn location_str(&self) -> String {
        match self.location() {
            Some(location) => {
                format!("{}:{}", self.file(), location)
            }
            None => format!("{}:{}", self.file(), self.position()),
        }
    }

    #[inline]
    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.position = position
    }

    #[inline]
    fn token_ahead(&self) -> &Option<Token<I>> {
        &self.token
    }

    #[inline]
    fn set_token_ahead(&mut self, token: Option<Token<I>>) {
        self.token = token;
    }

    #[inline]
    fn input(&self) -> &I {
        &self.input
    }

    #[inline]
    fn location(&self) -> &Option<Location> {
        &self.location
    }

    #[inline]
    fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    #[inline]
    fn layout(&self) -> &Option<I> {
        &self.layout
    }

    #[inline]
    fn set_layout(&mut self, layout: I) {
        self.layout = Some(layout);
    }
}

impl<I> LRContext<I> {
    pub fn state(&self) -> StateIndex {
        self.state
    }

    pub fn set_state(&mut self, state: StateIndex) {
        self.state = state
    }
}

impl<'i> LRContext<&'i str> {
    fn context_str(&self) -> String {
        self.input()
            [self.position() - min(15, self.position())..self.position()]
            .chars()
            .chain("-->".chars())
            .chain(self.input()[self.position()..].chars().take(15))
            .collect::<String>()
    }

    fn update_location<C: AsRef<str>>(&mut self, content: C) {
        let content = content.as_ref();
        let (mut line, mut column) = self.location().map_or((1, 0), |l| match l {
            Location{start: Position::LineBased(lb), ..} => (lb.line, lb.column),
            _ => panic!(),
        });
        let newlines = content.as_bytes().iter().filter(|&c| *c == b'\n').count();
        let newcolumn = content.len() - content.as_bytes().iter().rposition(|&c| c == b'\n').unwrap_or(0);
        line += newlines;
        column += newcolumn;

        self.set_location(Location {
            start: Position::LineBased(LineBased{line, column}),
            end: None,
        });
        self.set_position(self.position() + content.len());
    }
}

impl<I> From<&mut LRContext<I>> for Location {
    fn from(context: &mut LRContext<I>) -> Self {
        context.location().unwrap_or(Self {
            start: Position::Position(context.position()),
            end: None,
        })
    }
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct LRStringLexer<D: 'static> {
    pub(crate) definition: &'static D,
}

impl<D> LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    pub fn new(definition: &'static D) -> Self {
        Self { definition }
    }

    fn skip<'i>(context: &mut LRContext<&'i str>) {
        let skipped = context.input()[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .collect::<String>();
        log!("Skipped ws: {}", skipped.len());
        context.set_layout(
            &context.input()
                [context.position()..context.position() + skipped.len()],
        );
        context.update_location(skipped);
    }
}

impl<'i, D> Lexer<&'i str, LRContext<&'i str>> for LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    fn next_token(
        &self,
        context: &mut LRContext<&'i str>,
    ) -> RustemoResult<Token<&'i str>> {
        Self::skip(context);
        log!("Context: {}", context.context_str());
        let token: Option<Token<&'i str>> = self
            .definition
            .recognizers(context.state())
            .map(|(recognizer, terminal_info)| {
                (
                    recognizer(&context.input()[context.position()..]),
                    terminal_info,
                )
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, terminal_info)| Token {
                terminal: terminal_info,
                value: recognized.unwrap(),
                location: None,
                layout: None,
                layout_location: None,
            })
            // Take the first token or return None if no tokens are found.
            .next();

        match token {
            Some(t) => {
                context.update_location(t.value);
                context.set_token_ahead(Some(t.clone()));
                Ok(t)
            }
            None => {
                context.set_token_ahead(None);
                let expected = self
                    .definition
                    .recognizers(context.state())
                    .map(|(_, terminal_info)| terminal_info.name)
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(RustemoError::ParseError {
                    message: format!(
                        r#"Error at position {} "{}". Expected one of {}."#,
                        context.location_str(),
                        context.context_str(),
                        expected
                    ),
                    file: context.file(),
                    location: Location::from(context),
                })
            }
        }
    }
}

pub trait LexerDefinition {
    type Recognizer;
    /// For the given state, returns iterator of recognizers that should be
    /// tried in order.
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::Recognizer>;
}

pub struct RecognizerIterator<R: 'static> {
    pub terminals: &'static [TerminalInfo],
    pub terminals_for_state: &'static [Option<usize>],
    pub recognizers: &'static [R],
    pub index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = (&'static R, &'static TerminalInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.terminals_for_state.len() {
            match self.terminals_for_state[self.index] {
                Some(term_idx) => {
                    self.index += 1;
                    Some((
                        &self.recognizers[term_idx],
                        &self.terminals[term_idx],
                    ))
                }
                None => None,
            }
        } else {
            None
        }
    }
}
