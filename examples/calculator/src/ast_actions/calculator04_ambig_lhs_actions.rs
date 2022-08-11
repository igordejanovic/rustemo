///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EAdd {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct ESub {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EMul {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EDiv {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EPow {
    pub base: Box<E>,
    pub exp: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(EAdd),
    Sub(ESub),
    Mul(EMul),
    Div(EDiv),
    Pow(EPow),
    Paren(Box<E>),
    Num(Num),
}
pub fn e_add(left: E, right: E) -> E {
    E::Add(EAdd {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_sub(left: E, right: E) -> E {
    E::Sub(ESub {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_mul(left: E, right: E) -> E {
    E::Mul(EMul {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_div(left: E, right: E) -> E {
    E::Div(EDiv {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_pow(base: E, exp: E) -> E {
    E::Pow(EPow {
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
