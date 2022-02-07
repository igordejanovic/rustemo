use crate::{
    common::Location,
    grammar::TerminalInfo,
    parser::Context, index::TermIndex,
};
use core::fmt::Debug;

pub trait Lexer {
    /// Content stored in a Token. The type of the input.
    type Input: Debug + Clone;
    /// Given the current context should generate next token or None if no token
    /// is found.
    fn next_token(&mut self, context: &mut impl Context) -> Option<Token<Self::Input>>;
}

#[derive(Clone, Debug)]
pub struct Token<I> {
    pub terminal: &'static TerminalInfo,
    pub value: I,
    pub location: Option<Location>,
    pub layout: Option<I>,
    pub layout_location: Option<Location>,
}

impl<I> Token<I> {
    #[inline]
    pub fn index(&self) -> TermIndex {
        self.terminal.id
    }
}
