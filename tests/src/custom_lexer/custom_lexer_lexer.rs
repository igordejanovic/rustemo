use super::custom_lexer::TokenKind;
use rustemo_rt::{
    error::Result,
    index::StateIndex,
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
};

pub type Input = [u8];

pub struct CustomLexerLexer();

impl CustomLexerLexer {
    pub fn new() -> Self {
        CustomLexerLexer()
    }
}

impl<'i> Lexer<'i, Input, (), StateIndex, TokenKind> for CustomLexerLexer {
    fn next_token(
        &self,
        context: &mut Context<'i, Input, (), StateIndex>,
    ) -> Result<Token<'i, Input, TokenKind>> {
        let value;
        let kind: lexer::TokenKind<TokenKind>;
        let mut pos = context.position;
        if context.position >= context.input.len() {
            value = &[][..];
            kind = lexer::TokenKind::STOP;
        } else {
            while (context.input[pos] & 0b1000_0000) != 0 {
                pos += 1;
            }
            value = &context.input[context.position..=pos];
            kind = lexer::TokenKind::Kind(TokenKind::VarInt);
        }

        Ok(Token {
            kind,
            value,
            location: Some(Location {
                start: Position::Position(context.position),
                end: Some(Position::Position(pos)),
            }),
        })
    }
}
