use super::custom_lexer_1::{State, TokenKind};
use rustemo::{Context, LRContext, Lexer, Location, Position, Result, Token};
use std::iter;

/// We are parsing a slice of bytes.
pub type Input = [u8];
pub type Ctx<'i> = LRContext<'i, Input, State, TokenKind>;

pub struct MyCustomLexer1();

impl MyCustomLexer1 {
    pub fn new() -> Self {
        MyCustomLexer1()
    }
}

/// This custom lexer will recognize a VarInt in the input by returning a slice
/// of the input where first bytes has highest bit set while the last byte
/// highest bit is .
impl<'i> Lexer<'i, Ctx<'i>, State, TokenKind> for MyCustomLexer1 {
    type Input = Input;

    fn next_tokens(
        &self,
        context: &mut Ctx<'i>,
        input: &'i Self::Input,
        _token_kinds: Vec<TokenKind>,
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TokenKind>> + 'i> {
        let value;
        let kind: TokenKind;
        let mut pos = context.position();
        if context.position() >= input.len() {
            value = &[][..];
            kind = TokenKind::STOP;
        } else {
            // Increase position as long as the highest bit is set.
            while (input[pos] & 0b1000_0000) != 0 {
                pos += 1;
            }
            // Token value is the slice of the input where VarInt is reconized.
            value = &input[context.position()..=pos];
            kind = TokenKind::VarInt;
        }

        Box::new(iter::once(Token {
            kind,
            value,
            location: Location {
                start: Position::Position(context.position()),
                end: Some(Position::Position(pos)),
            },
        }))
    }
}
