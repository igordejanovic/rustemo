///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer;
use super::calculator04_ambig_lhs::TokenKind;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, str, TokenKind>;
pub type Num = String;
pub fn num<'i>(token: Token<'i>) -> Num {
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
pub struct Pow {
    pub base: Box<E>,
    pub exp: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Pow(Pow),
    Paren(Box<E>),
    Num(Num),
}
pub fn e_add(left: E, right: E) -> E {
    E::Add(Add {
        left: Box::new(left),
        right: Box::new(right),
    })
}
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
pub fn e_pow(base: E, exp: E) -> E {
    E::Pow(Pow {
        base: Box::new(base),
        exp: Box::new(exp),
    })
}
pub fn e_paren(e: E) -> E {
    E::Paren(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::Num(num)
}
