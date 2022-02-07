use crate::{builder::Builder, lexer::Lexer, index::{StateIndex, TermIndex, NonTermIndex, ProdIndex}};
use core::fmt::Debug;

pub trait Parser<L, B>
where
    L: Lexer,
    B: Builder<Lexer = L>,
{
    fn parse(&mut self, lexer: L) -> B::Output;
}

pub trait ParserDefinition {
    fn action(&self, state: StateIndex, term_index: TermIndex) -> Action;
    fn goto(&self, state: StateIndex, nonterm_id: NonTermIndex) -> StateIndex;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Shift(StateIndex, TermIndex),
    Reduce(ProdIndex, usize, NonTermIndex, &'static str),
    Accept,
    Error,
}

/// Parser context provides necessary information to lexers and actions.
pub trait Context {
    fn position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn state(&self) -> StateIndex;
}
