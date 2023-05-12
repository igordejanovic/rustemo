use super::custom_lexer_1::{TokenKind, TokenRecognizer};
use rustemo::{
    error::Result,
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
};

/// We are parsing a slice of bytes.
pub type Input = [u8];

pub struct MyCustomLexer1();

impl MyCustomLexer1 {
    pub fn new() -> Self {
        MyCustomLexer1()
    }
}

/// This custom lexer will recognize a VarInt in the input by returning a slice
/// of the input where first bytes has highest bit set while the last byte
/// highest bit is .
impl Lexer<Input, TokenRecognizer> for MyCustomLexer1 {
    fn next_token<'i>(
        &self,
        context: &mut Context<'i, Input>,
        _token_recognizers: &[&TokenRecognizer],
    ) -> Option<Token<'i, Input, TokenKind>> {
        let value;
        let kind: TokenKind;
        let mut pos = context.position;
        if context.position >= context.input.len() {
            value = &[][..];
            kind = TokenKind::STOP;
        } else {
            // Increase position as long as the highest bit is set.
            while (context.input[pos] & 0b1000_0000) != 0 {
                pos += 1;
            }
            // Token value is the slice of the input where VarInt is reconized.
            value = &context.input[context.position..=pos];
            kind = TokenKind::VarInt;
        }

        Some(Token {
            kind,
            value,
            location: Location {
                start: Position::Position(context.position),
                end: Some(Position::Position(pos)),
            },
        })
    }
}
