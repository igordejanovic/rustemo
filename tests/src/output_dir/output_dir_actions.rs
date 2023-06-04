/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::output_dir::Context;
use super::output_dir::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct A {
    pub b1: B1,
    pub num: Num,
}
pub fn a_c1(_ctx: &Context, b1: B1, num: Num) -> A {
    A { b1, num }
}
pub type B1 = Vec<B>;
pub fn b1_c1(_ctx: &Context, mut b1: B1, b: B) -> B1 {
    b1.push(b);
    b1
}
pub fn b1_b(_ctx: &Context, b: B) -> B1 {
    vec![b]
}
#[derive(Debug, Clone)]
pub enum B {
    Tb,
}
pub fn b_tb(_ctx: &Context) -> B {
    B::Tb
}
