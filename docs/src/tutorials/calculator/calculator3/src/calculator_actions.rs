use super::calculator::Context;
use super::calculator::TokenKind;
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Number = String;
pub fn number<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Number {
    token.value.into()
}
/// ANCHOR: structs
#[derive(Debug, Clone)]
pub struct Add {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Sub {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
/// ANCHOR_END: structs
#[derive(Debug, Clone)]
pub struct Mul {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Div {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Number(Number),
}
pub fn e_add(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::Add(Add {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_sub(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::Sub(Sub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::Mul(Mul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::Div(Div {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_number(_ctx: &Context, number: Number) -> E {
    E::Number(number)
}
