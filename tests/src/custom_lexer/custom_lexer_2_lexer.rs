use super::custom_lexer_2::TokenKind;
use rustemo_rt::{
    error::Result,
    index::StateIndex,
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
};

pub type Input = [u8];

pub struct CustomLexer2Lexer();

impl CustomLexer2Lexer {
    pub fn new() -> Self {
        CustomLexer2Lexer()
    }
}

impl<'i> Lexer<'i, Input, (), StateIndex, TokenKind> for CustomLexer2Lexer {
    fn next_token(
        &self,
        context: &mut Context<'i, Input, (), StateIndex>,
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
