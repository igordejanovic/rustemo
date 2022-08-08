///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Add {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Sub {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Mul {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Div {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Pow {
    pub base: Box<E>,
    pub exp: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum E {
    E_Add(E_Add),
    E_Sub(E_Sub),
    E_Mul(E_Mul),
    E_Div(E_Div),
    E_Pow(E_Pow),
    E_Paren(Box<E>),
    E_Num(Num),
}
pub fn e_add(left: E, right: E) -> E {
    E::E_Add(E_Add {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_sub(left: E, right: E) -> E {
    E::E_Sub(E_Sub {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_mul(left: E, right: E) -> E {
    E::E_Mul(E_Mul {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_div(left: E, right: E) -> E {
    E::E_Div(E_Div {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_pow(base: E, exp: E) -> E {
    E::E_Pow(E_Pow {
        base: Box::new(base),
        exp: Box::new(exp),
    })
}
pub fn e_paren(e: E) -> E {
    E::E_Paren(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::E_Num(num)
}
