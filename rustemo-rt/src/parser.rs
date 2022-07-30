use crate::{
    builder::Builder,
    error::RustemoResult,
    lexer::{Context, Lexer},
};

pub trait Parser<I, C, L, B>
where
    C: Context<I>,
    L: Lexer<I, C>,
    B: Builder,
{
    fn parse(
        &mut self,
        context: C,
        lexer: L,
        builder: B,
    ) -> RustemoResult<B::Output>;
}
