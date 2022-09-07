use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Input, Lexer},
};

pub trait Parser<'i, I, L, B, LO, ST, TK>
where
    I: Input<'i>,
    L: Lexer<'i, I, LO, ST, TK>,
    B: Builder,
    TK: Copy,
{
    fn parse(
        &mut self,
        context: &mut Context<I, LO, ST>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output>;
}
