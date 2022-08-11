///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EV1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EV2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EV3 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EV4 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EV5 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    V1(EV1),
    V2(EV2),
    V3(EV3),
    V4(EV4),
    V5(EV5),
    V6(Box<E>),
    Num(Num),
}
pub fn e_v1(e_1: E, e_3: E) -> E {
    E::V1(EV1 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_v2(e_1: E, e_3: E) -> E {
    E::V2(EV2 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_v3(e_1: E, e_3: E) -> E {
    E::V3(EV3 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_v4(e_1: E, e_3: E) -> E {
    E::V4(EV4 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_v5(e_1: E, e_3: E) -> E {
    E::V5(EV5 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_v6(e: E) -> E {
    E::V6(Box::new(e))
}
pub fn e_num(num: Num) -> E {
    E::Num(num)
}
