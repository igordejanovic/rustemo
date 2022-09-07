use rustemo_rt::{
    builder::Builder,
    index::{ProdIndex, StateIndex, TermIndex},
    lexer::{self, Token},
    lr::builder::LRBuilder,
};

use super::custom_builder::{ProdKind, TokenKind};

pub type E = i32;
pub type Context<'i> = lexer::Context<'i, str, (), StateIndex>;

/// Custom builder that perform arithmetic operations.
pub struct CustomBuilderBuilder {
    // A stack used to shift numbers and keep intermediate result
    stack: Vec<i32>,
}

impl Builder for CustomBuilderBuilder {
    // Result of the build process will be a number
    type Output = E;

    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn get_result(&mut self) -> Self::Output {
        assert!(self.stack.len() == 1);
        self.stack.pop().unwrap()
    }
}

impl<'i> LRBuilder<'i, str, (), TokenKind>
    for CustomBuilderBuilder
{
    fn shift_action(
        &mut self,
        _context: &Context<'i>,
        token: Token<'i, str, TokenKind>,
    ) {
        let kind = match token.kind {
            lexer::TokenKind::Kind(kind) => kind,
            lexer::TokenKind::STOP => panic!("Cannot shift STOP token!"),
        };
        match kind {
            // We are interested only in parsing numbers as operations will be
            // handled by reductions.
            TokenKind::Num => self.stack.push(token.value.parse().unwrap()),
            _ => (),
        }
    }

    fn reduce_action(
        &mut self,
        _context: &Context<'i>,
        prod_idx: ProdIndex,
        _prod_len: usize,
    ) {
        let res = match ProdKind::from(prod_idx) {
            ProdKind::EAdd => {
                self.stack.pop().unwrap() + self.stack.pop().unwrap()
            }
            ProdKind::EMul => {
                self.stack.pop().unwrap() * self.stack.pop().unwrap()
            }
            ProdKind::ENum => self.stack.pop().unwrap(),
        };
        self.stack.push(res);
    }
}
