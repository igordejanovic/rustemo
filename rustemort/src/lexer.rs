use crate::{
    common::Location,
    grammar::TerminalInfo,
    parser::Context, index::TermIndex,
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
    fn next_token(&self, context: &mut impl Context<Self::Input>) -> Option<Token<Self::Input>>;
}

/// `Token` represent a single token from the input stream.
#[derive(Clone, Debug)]
pub struct Token<I> {
    pub terminal: &'static TerminalInfo,
    /// The part of the input stream that this token represents.
    pub value: I,
    /// Location (with span) in the input file where this token is found.
    pub location: Option<Location>,
    /// The semantically irrelevant part of the input (e.g. whitespaces) the
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
