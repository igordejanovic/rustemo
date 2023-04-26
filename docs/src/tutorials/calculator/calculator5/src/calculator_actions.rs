///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::calculator::Context;
use super::calculator::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Number = String;
pub fn number<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Number {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Add {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Sub {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Mul {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Div {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Number(Number),
}
pub fn e_add(_ctx: &Context, left: E, right: E) -> E {
    E::Add(Add {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_sub(_ctx: &Context, left: E, right: E) -> E {
    E::Sub(Sub {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_mul(_ctx: &Context, left: E, right: E) -> E {
    E::Mul(Mul {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_div(_ctx: &Context, left: E, right: E) -> E {
    E::Div(Div {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_number(_ctx: &Context, number: Number) -> E {
    E::Number(number)
}
