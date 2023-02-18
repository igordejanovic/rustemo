use super::custom_lexer_2::TokenKind;
use rustemo::{
    error::Result,
    index::StateIndex,
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
};

/// We are parsing a slice of bytes.
pub type Input = [u8];

pub struct CustomLexer2Lexer();

impl CustomLexer2Lexer {
    pub fn new() -> Self {
        CustomLexer2Lexer()
    }
}

/// In this custom lexer we are not recognizing a full VarInts but only its
/// constituents: MSBByte (if highest bit is set), NonMSBByte (highest bit is
/// not set). How these bytes is organized into VarInts is defined by the
/// grammar and the transformation to a numeric value is done in actions.
impl<'i> Lexer<'i, Input, StateIndex, TokenKind> for CustomLexer2Lexer {
    fn next_token(
        &self,
        context: &mut Context<'i, Input, StateIndex>,
    ) -> Result<Token<'i, Input, TokenKind>> {
        let value;
        let kind: lexer::TokenKind<TokenKind>;
        if context.position >= context.input.len() {
            value = &[][..];
            kind = lexer::TokenKind::STOP;
        } else {
            value = &context.input[context.position..=context.position];
            if value[0] & 0b1000_0000 != 0 {
                kind = lexer::TokenKind::Kind(TokenKind::MSBByte);
            } else {
                kind = lexer::TokenKind::Kind(TokenKind::NonMSBByte);
            };
        }

        Ok(Token {
            kind,
            value,
            location: Some(Location {
                start: Position::Position(context.position),
                end: Some(Position::Position(context.position)),
            }),
        })
    }
}
