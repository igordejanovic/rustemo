use super::custom_lexer_2::{TokenKind, TokenRecognizer};
use rustemo::{
    error::Result,
    index::StateIndex,
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
};

// ANCHOR: custom-lexer
/// We are parsing a slice of bytes.
pub type Input = [u8];

pub struct MyCustomLexer2();

impl MyCustomLexer2 {
    pub fn new() -> Self {
        MyCustomLexer2()
    }
}

impl lexer::TokenRecognizer for TokenRecognizer {
    type TokenKind = TokenKind;
    type Input = Input;

    fn token_kind(&self) -> Self::TokenKind {
        self.token_kind
    }
}

/// In this custom lexer we are not recognizing a full VarInts but only its
/// constituents: MSBByte (if highest bit is set), NonMSBByte (highest bit is
/// not set). How these bytes is organized into VarInts is defined by the
/// grammar and the transformation to a numeric value is done in actions.
impl Lexer<Input, TokenRecognizer> for MyCustomLexer2 {
    fn next_token<'i>(
        &self,
        context: &mut Context<'i, Input>,
        _token_recognizers: &[&TokenRecognizer],
    ) -> Option<Token<'i, Input, TokenKind>> {
        let value;
        let kind: TokenKind;
        if context.position >= context.input.len() {
            value = &[][..];
            kind = TokenKind::STOP;
        } else {
            value = &context.input[context.position..=context.position];
            if value[0] & 0b1000_0000 != 0 {
                kind = TokenKind::MSBByte;
            } else {
                kind = TokenKind::NonMSBByte;
            };
        }

        Some(Token {
            kind,
            value,
            location: Location {
                start: Position::Position(context.position),
                end: Some(Position::Position(context.position)),
            },
        })
    }
}
// ANCHOR_END: custom-lexer
