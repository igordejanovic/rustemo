use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Lexer},
};

pub trait Parser<I, C, L, B>
where
    C: Context<I>,
    L: Lexer<I, C>,
    B: Builder,
{
    fn parse(&mut self, context: C, lexer: L, builder: B) -> Result<B::Output>;
}
