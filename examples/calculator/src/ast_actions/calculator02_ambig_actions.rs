use super::calculator02_ambig::Context;
use super::calculator02_ambig::TokenKind;
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(_ctx: &Context, token: Token) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EC1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC3 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC4 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC5 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    C1(EC1),
    C2(EC2),
    C3(EC3),
    C4(EC4),
    C5(EC5),
    E(Box<E>),
    Num(Num),
}
pub fn e_c1(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::C1(EC1 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c2(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::C2(EC2 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c3(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::C3(EC3 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c4(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::C4(EC4 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c5(_ctx: &Context, e_1: E, e_3: E) -> E {
    E::C5(EC5 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_e(_ctx: &Context, e: E) -> E {
    E::E(Box::new(e))
}
pub fn e_num(_ctx: &Context, num: Num) -> E {
    E::Num(num)
}
