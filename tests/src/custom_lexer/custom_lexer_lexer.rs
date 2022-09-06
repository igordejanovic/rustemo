use rustemo_rt::{lexer::{Context, Lexer, Token}, index::StateIndex, location::Location};

pub struct CustomLexerLexer();

impl CustomLexerLexer {
    pub fn new() -> Self {
        CustomLexerLexer()
    }
}

impl Lexer for CustomLexerLexer {
    fn next_token(&self, context: &mut Context<[u8], (), StateIndex>) -> Token<[u8]> {
        let pos = context.position;
        while context.input[pos] & 0b1000_0000 {
            pos+=1;
        }
        let value = context.input[context.position..=pos];

        Token{
            terminal: ,
            value,
            location: Some(Location { start: context.position, end: Some(pos) }),
        }
    }
}
