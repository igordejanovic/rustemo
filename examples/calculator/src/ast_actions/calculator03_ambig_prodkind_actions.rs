///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer;
use super::calculator03_ambig_prodkind::TokenKind;
pub type Token<'i> = lexer::Token<&'i str, TokenKind>;
pub type Num = String;
pub fn num<'i>(token: Token<'i>) -> Num {
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
pub fn e_add(e_1: E, e_3: E) -> E {
    E::Add(Add {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_sub(e_1: E, e_3: E) -> E {
    E::Sub(Sub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(e_1: E, e_3: E) -> E {
    E::Mul(Mul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(e_1: E, e_3: E) -> E {
    E::Div(Div {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_pow(e_1: E, e_3: E) -> E {
    E::Pow(Pow {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_paren(e: E) -> E {
    E::Paren(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::Num(num)
}
