use super::pass_context::{Context, TokenKind};
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer::Token;
pub type Num = u32;
pub fn num<'i>(context: &Context<'i>, token: Token<'i, str, TokenKind>) -> Num {
    token.value.parse::<u32>().unwrap() + context.position as u32
}
pub type A = Num1;
pub fn a_c1<'i>(_context: &Context<'i>, num1: Num1) -> A {
    num1
}
pub type Num1 = Vec<Num>;
pub fn num1_c1<'i>(_context: &Context<'i>, mut num1: Num1, num: Num) -> Num1 {
    num1.push(num);
    num1
}
pub fn num1_c2<'i>(_context: &Context<'i>, num: Num) -> Num1 {
    vec![num]
}
