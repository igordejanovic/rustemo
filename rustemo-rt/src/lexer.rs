use crate::{
    common::Location,
    debug::log,
    grammar::TerminalInfo,
    index::{StateIndex, TermIndex},
    parser::Context,
};
use core::fmt::Debug;

/// The `Lexer` trait allows input tokenization
///
/// Lexer is stateless and its job is to produce next token given the current
/// context.
pub trait Lexer {
    /// The type of the input that is being parsed.
    type Input;
    /// Given the current context, this method should generate next token or
    /// None if no token is found. It should update the given mutable context to
    /// reflect the current progress.
    fn next_token(
        &self,
        context: &mut impl Context<Self::Input>,
    ) -> Option<Token<Self::Input>>;
}

/// `Token` represent a single token from the input stream.
#[derive(Clone, Debug)]
pub struct Token<I> {
    pub terminal: &'static TerminalInfo,
    /// The part of the input stream that this token represents.
    pub value: I,
    /// Location (with span) in the input file where this token is found.
    pub location: Option<Location>,
    /// Semantically irrelevant part of the input (e.g. whitespaces) the
    /// preceeds the token.
    pub layout: Option<I>,
    /// Location (with span) in the input file where this layout is found.
    pub layout_location: Option<Location>,
}

impl<I> Token<I> {
    #[inline]
    pub fn index(&self) -> TermIndex {
        self.terminal.id
    }
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct DefaultLexer<'i, D: 'static> {
    pub(crate) input: &'i str,
    pub(crate) token_ahead: Option<Token<&'i str>>,
    pub(crate) definition: &'static D,
}

impl<'i, D> DefaultLexer<'i, D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    pub fn new(input: &'i str, definition: &'static D) -> Self {
        Self {
            input,
            token_ahead: None,
            definition,
        }
    }
    fn skip<I>(&self, context: &mut impl Context<I>) {
        let skipped = self.input[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .collect::<String>();
        log!("Skipped ws: {}", skipped.len());
        context.set_position(context.position() + skipped.len());
    }
}

impl<'i, D> Lexer for DefaultLexer<'i, D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    type Input = &'i str;
    fn next_token(
        &self,
        context: &mut impl Context<Self::Input>,
    ) -> Option<Token<Self::Input>> {
        self.skip(context);
        log!(
            "Context: {}",
            self.input.chars().take(30).collect::<String>()
        );
        let token: Option<Token<&'i str>> = self
            .definition
            .recognizers(context.state())
            .map(|(recognizer, terminal_info)| {
                (recognizer(&self.input[context.position()..]), terminal_info)
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
                let new_pos = context.position() + t.value.len();
                context.set_position(new_pos);
                context.set_token_ahead(Some(t.clone()));
                Some(t)
            }
            None => {
                context.set_token_ahead(None);
                None
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
