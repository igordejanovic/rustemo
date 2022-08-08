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
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Sub {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Mul {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Div {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_Pow {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
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
pub fn e_add(e_1: E, e_3: E) -> E {
    E::E_Add(E_Add {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_sub(e_1: E, e_3: E) -> E {
    E::E_Sub(E_Sub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(e_1: E, e_3: E) -> E {
    E::E_Mul(E_Mul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(e_1: E, e_3: E) -> E {
    E::E_Div(E_Div {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_pow(e_1: E, e_3: E) -> E {
    E::E_Pow(E_Pow {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_paren(e: E) -> E {
    E::E_Paren(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::E_Num(num)
}
