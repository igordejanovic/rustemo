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
    pub e: Box<E>,
    pub t: T,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum E {
    E_1(E_1),
    E_2(T),
}
pub fn e_1(e: E, t: T) -> E {
    E::E_1(E_1 { e: Box::new(e), t })
}
pub fn e_2(t: T) -> E {
    E::E_2(t)
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct T_1 {
    pub t: Box<T>,
    pub f: F,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum T {
    T_1(T_1),
    T_2(F),
}
pub fn t_1(t: T, f: F) -> T {
    T::T_1(T_1 { t: Box::new(t), f })
}
pub fn t_2(f: F) -> T {
    T::T_2(f)
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum F {
    F_1(Box<E>),
    F_2(Num),
}
pub fn f_1(e: E) -> F {
    F::F_1(Box::new(e))
}
pub fn f_2(num: Num) -> F {
    F::F_2(num)
}
