use super::custom_lexer::TokenKind;
use rustemo_rt::{
    error::Result,
    index::StateIndex,
    lexer::{self, Context, Input, Lexer, Token},
    location::{Location, Position},
};

pub struct CustomLexerLexer();

impl CustomLexerLexer {
    pub fn new() -> Self {
        CustomLexerLexer()
    }
}

impl<'i> Lexer<'i, [u8], (), StateIndex, TokenKind> for CustomLexerLexer {
    fn next_token(
        &self,
        context: &mut Context<'i, [u8], (), StateIndex>,
    ) -> Result<Token<'i, [u8], TokenKind>> {
        let value;
        let kind: lexer::TokenKind<TokenKind>;
        let pos = context.position;
        if context.position >= context.input.len() {
            value = &[][..];
            kind = lexer::TokenKind::STOP;
        } else {
            while (context.input[pos] & 0b1000_0000) != 0 {
                pos += 1;
            }
            let value = &context.input[context.position..=pos];
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
