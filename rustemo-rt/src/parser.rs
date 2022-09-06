use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Input, Lexer},
};

pub trait Parser<I, L, B, LO, ST, TK>
where
    I: Input,
    L: Lexer<I, LO, ST, TK>,
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
