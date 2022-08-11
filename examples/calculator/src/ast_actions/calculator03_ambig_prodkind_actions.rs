///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EAdd {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct ESub {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EMul {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EDiv {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EPow {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
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
pub fn e_add(e_1: E, e_3: E) -> E {
    E::Add(EAdd {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_sub(e_1: E, e_3: E) -> E {
    E::Sub(ESub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(e_1: E, e_3: E) -> E {
    E::Mul(EMul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(e_1: E, e_3: E) -> E {
    E::Div(EDiv {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_pow(e_1: E, e_3: E) -> E {
    E::Pow(EPow {
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
