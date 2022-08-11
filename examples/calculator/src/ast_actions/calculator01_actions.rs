///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = String;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EV1 {
    pub e: Box<E>,
    pub t: T,
}
#[derive(Debug, Clone)]
pub enum E {
    V1(EV1),
    V2(T),
}
pub fn e_v1(e: E, t: T) -> E {
    E::V1(EV1 { e: Box::new(e), t })
}
pub fn e_v2(t: T) -> E {
    E::V2(t)
}
#[derive(Debug, Clone)]
pub struct TV1 {
    pub t: Box<T>,
    pub f: F,
}
#[derive(Debug, Clone)]
pub enum T {
    V1(TV1),
    V2(F),
}
pub fn t_v1(t: T, f: F) -> T {
    T::V1(TV1 { t: Box::new(t), f })
}
pub fn t_v2(f: F) -> T {
    T::V2(f)
}
#[derive(Debug, Clone)]
pub enum F {
    V1(Box<E>),
    V2(Num),
}
pub fn f_v1(e: E) -> F {
    F::V1(Box::new(e))
}
pub fn f_v2(num: Num) -> F {
    F::V2(num)
}
