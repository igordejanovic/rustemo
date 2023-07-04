use super::calculator03_ambig_prodkind::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.into()
}
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
#[derive(Debug, Clone)]
pub struct Mul {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
/// ANCHOR: named-matches
#[derive(Debug, Clone)]
pub struct Div {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Pow {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
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
pub fn e_add(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Add(Add {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
/// ANCHOR_END: named-matches
pub fn e_sub(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Sub(Sub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Mul(Mul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Div(Div {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_pow(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Pow(Pow {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_paren(_ctx: &Ctx, e: E) -> E {
    E::Paren(Box::new(e))
}
pub fn e_num(_ctx: &Ctx, num: Num) -> E {
    E::Num(num)
}
