use super::custom_lexer_2::{State, TokenKind};
use rustemo::{Context, LRContext, Lexer, Position, Result, SourceSpan, Token};
use std::iter;

// ANCHOR: custom-lexer
/// We are parsing a slice of bytes.
pub type Input = [u8];
pub type Ctx<'i> = LRContext<'i, Input, State, TokenKind>;

pub struct MyCustomLexer2();

impl MyCustomLexer2 {
    pub fn new() -> Self {
        MyCustomLexer2()
    }
}

/// In this custom lexer we are not recognizing a full VarInts but only its
/// constituents: MSBByte (if highest bit is set), NonMSBByte (highest bit is
/// not set). How these bytes is organized into VarInts is defined by the
/// grammar and the transformation to a numeric value is done in actions.
impl<'i> Lexer<'i, Ctx<'i>, State, TokenKind> for MyCustomLexer2 {
    type Input = Input;

    fn next_tokens(
        &self,
        context: &mut Ctx<'i>,
        input: &'i Self::Input,
        _token_kinds: Vec<(TokenKind, bool)>,
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TokenKind>> + 'i> {
        let value;
        let kind: TokenKind;
        if context.position().pos >= input.len() {
            value = &[][..];
            kind = TokenKind::STOP;
        } else {
            value = &input[context.position().pos..=context.position().pos];
            if value[0] & 0b1000_0000 != 0 {
                kind = TokenKind::MSBByte;
            } else {
                kind = TokenKind::NonMSBByte;
            };
        }

        Box::new(iter::once(Token {
            kind,
            value,
            span: SourceSpan {
                start: context.position(),
                end: context.position(),
            },
        }))
    }
}
// ANCHOR_END: custom-lexer
