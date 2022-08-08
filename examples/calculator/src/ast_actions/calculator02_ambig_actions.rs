///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_3 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_4 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct E_5 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum E {
    E_1(E_1),
    E_2(E_2),
    E_3(E_3),
    E_4(E_4),
    E_5(E_5),
    E_6(Box<E>),
    E_Num(Num),
}
pub fn e_1(e_1: E, e_3: E) -> E {
    E::E_1(E_1 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_2(e_1: E, e_3: E) -> E {
    E::E_2(E_2 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_3(e_1: E, e_3: E) -> E {
    E::E_3(E_3 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_4(e_1: E, e_3: E) -> E {
    E::E_4(E_4 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_5(e_1: E, e_3: E) -> E {
    E::E_5(E_5 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_6(e: E) -> E {
    E::E_6(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::E_Num(num)
}
