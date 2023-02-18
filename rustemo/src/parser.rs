use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Input, Lexer},
};

pub trait Parser<'i, I, L, B, ST, TK>
where
    I: Input + ?Sized,
    L: Lexer<'i, I, ST, TK>,
    B: Builder,
    TK: Copy,
{
    fn parse(
        &mut self,
        context: &mut Context<'i, I, ST>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output>;
}
