use rustemo_rt::{lexer::{self, Context, Lexer, Token, Input}, index::StateIndex, location::Location};
use super::custom_lexer::TokenKind;

pub struct CustomLexerLexer();

impl CustomLexerLexer {
    pub fn new() -> Self {
        CustomLexerLexer()
    }
}

pub struct Bytes<'i>(&'i [u8]);

impl Input for Bytes {
}

impl<'i> Lexer<Bytes<'i>, (), StateIndex, TokenKind> for CustomLexerLexer {
    fn next_token(&self, context: &mut Context<Bytes<'i>, (), StateIndex>) -> Token<Bytes<'i>, TokenKind> {
        let value;
        let kind;
        let pos = context.position;
        if context.position >= context.input.0.len() {
            value = &[];
            kind = lexer::TokenKind::STOP;
        } else {
            while context.input.0[pos] & 0b1000_0000 {
                pos+=1;
            }
            let value = context.input.0[context.position..=pos];
        }

        Token{
            kind: lexer::TokenKind::Kind(TokenKind::VarInt),
            value,
            location: Some(Location { start: context.position, end: Some(pos) }),
        }
    }
}
