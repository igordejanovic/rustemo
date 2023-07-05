/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::use_context::{self, TokenKind};
use rustemo::context::Context;
use rustemo::lexer;
pub type Input = str;
pub type Ctx<'i> = use_context::Context<'i, Input>;
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = u32;
pub fn num(context: &Ctx, token: Token) -> Num {
    token.value.parse::<u32>().unwrap() + context.position() as u32
}
pub type A = Num1;
pub fn a_num1(_ctx: &Ctx, num1: Num1) -> A {
    num1
}
pub type Num1 = Vec<Num>;
pub fn num1_c1(_context: &Ctx, mut num1: Num1, num: Num) -> Num1 {
    num1.push(num);
    num1
}
pub fn num1_num(_ctx: &Ctx, num: Num) -> Num1 {
    vec![num]
}
