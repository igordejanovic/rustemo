use crate::{
    builder::Builder,
    lexer::{Context, Lexer}, error::RustemoResult,
};

pub trait Parser<I, C, L, B>
where
    C: Context<I>,
    L: Lexer<I, C>,
    B: Builder,
{
    fn parse(&mut self, context: C, lexer: L, builder: B) -> RustemoResult<B::Output>;
}

