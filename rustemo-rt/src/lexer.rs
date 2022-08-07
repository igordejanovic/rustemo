use crate::{
    error::Result, grammar::TerminalInfo, index::TermIndex, location::Location,
};
use core::fmt::Debug;

/// The `Lexer` trait allows input tokenization
///
/// Lexer is stateless and its job is to produce next token given the current
/// context.
pub trait Lexer<I, C: Context<I>> {
    /// Given the current context, this method should return RustemoResult with
    /// token found ahead of the current location or error indicating what is
    /// expected.
    fn next_token(&self, context: &mut C) -> Result<Token<I>>;
}

/// Lexer context is used to keep the lexing state. It provides necessary
/// information to parsers and actions.
pub trait Context<I> {
    /// File path of the parsed content. "<str>" In case of static string.
    fn file(&self) -> String;

    /// The input being parsed. Should be set when the context is created.
    fn input(&self) -> &I;

    /// An absolute position in the input sequence
    ///
    /// The input must be indexable type.
    fn position(&self) -> usize;
    fn set_position(&mut self, position: usize);

    /// Location in the input if the input is line/column based.
    ///
    /// The location should be tracked by lexer but the lexers are stateless so
    /// the current location is always kept in the context. As location tracking
    /// incurs an overhead it should be configurable. In case of an error, lexer
    /// can calculate location information based on the absolute position.
    fn location(&self) -> &Option<Location>;
    fn set_location(&mut self, location: Location);

    /// A full location string containing the file being parsed together with
    /// the current location or absolute position if location is not used.
    fn location_str(&self) -> String;

    /// Layout before the current token ahead (e.g. whitespaces)
    fn layout(&self) -> &Option<I>;
    fn set_layout(&mut self, layout: I);
}

/// `Token` represent a single token from the input stream.
#[derive(Debug)]
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
