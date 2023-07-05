/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::output_dir::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct A {
    pub b1: B1,
    pub num: Num,
}
pub fn a_c1(_ctx: &Ctx, b1: B1, num: Num) -> A {
    A { b1, num }
}
pub type B1 = Vec<B>;
pub fn b1_c1(_ctx: &Ctx, mut b1: B1, b: B) -> B1 {
    b1.push(b);
    b1
}
pub fn b1_b(_ctx: &Ctx, b: B) -> B1 {
    vec![b]
}
#[derive(Debug, Clone)]
pub enum B {
    Tb,
}
pub fn b_tb(_ctx: &Ctx) -> B {
    B::Tb
}
