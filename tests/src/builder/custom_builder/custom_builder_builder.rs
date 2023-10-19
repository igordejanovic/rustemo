use rustemo::{Builder, LRBuilder, LRContext, Token};

use super::custom_builder::{ProdKind, State, TokenKind};

// ANCHOR: custom-builder-base
pub type E = i32;
pub type Context<'i> = LRContext<'i, str, State, TokenKind>;

/// Custom builder that perform arithmetic operations.
pub struct MyCustomBuilder {
    // A stack used to shift numbers and keep intermediate result
    stack: Vec<E>,
}

impl MyCustomBuilder {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }
}

impl Builder for MyCustomBuilder {
    // Result of the build process will be a number
    type Output = E;

    fn get_result(&mut self) -> Self::Output {
        assert!(self.stack.len() == 1);
        self.stack.pop().unwrap()
    }
}
// ANCHOR_END: custom-builder-base

// ANCHOR: custom-builder-lr
impl<'i> LRBuilder<'i, str, Context<'i>, State, ProdKind, TokenKind>
    for MyCustomBuilder
{
    fn shift_action(
        &mut self,
        _context: &mut Context<'i>,
        token: Token<'i, str, TokenKind>,
    ) {
        if let TokenKind::Num = token.kind {
            self.stack.push(token.value.parse().unwrap())
        }
    }

    fn reduce_action(
        &mut self,
        _context: &mut Context<'i>,
        prod: ProdKind,
        _prod_len: usize,
    ) {
        let res = match prod {
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
// ANCHOR_END: custom-builder-lr
