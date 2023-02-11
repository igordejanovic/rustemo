///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::calculator::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
/// ANCHOR: number
pub type Number = String;
/// ANCHOR_END: number
/// ANCHOR: number_action
pub fn number(token: Token) -> Number {
    token.value.into()
}
/// ANCHOR_END: number_action
/// ANCHOR: structs
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
/// ANCHOR_END: structs
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
/// ANCHOR: expression
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    C5(Number),
}
/// ANCHOR_END: expression
/// ANCHOR: add_action
pub fn e_add(left: E, right: E) -> E {
    E::Add(Add {
        left: Box::new(left),
        right: Box::new(right),
    })
}
/// ANCHOR_END: add_action
pub fn e_sub(left: E, right: E) -> E {
    E::Sub(Sub {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_mul(left: E, right: E) -> E {
    E::Mul(Mul {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_div(left: E, right: E) -> E {
    E::Div(Div {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_c5(number: Number) -> E {
    E::C5(number)
}
