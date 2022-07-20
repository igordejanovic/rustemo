use crate::{
    builder::Builder,
    index::StateIndex,
    lexer::{Lexer, Token},
};

pub trait Parser<L, B>
where
    L: Lexer,
    B: Builder<Lexer = L>,
{
    fn parse(&mut self, lexer: L, builder: B) -> B::Output;
}

/// Parser context provides necessary information to lexers and actions.
pub trait Context<I> {
    fn position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn token_ahead(&self) -> &Option<Token<I>>;
    fn set_token_ahead(&mut self, token: Option<Token<I>>);
    fn state(&self) -> StateIndex;
}
