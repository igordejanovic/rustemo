use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Input, Lexer, TokenRecognizer},
};

pub trait Parser<'i, I, L, B, TR>
where
    I: Input + ?Sized,
    L: Lexer<I, TR>,
    B: Builder,
    TR: TokenRecognizer,
{
    fn parse(
        &mut self,
        context: &mut Context<'i, I>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output>;
}
