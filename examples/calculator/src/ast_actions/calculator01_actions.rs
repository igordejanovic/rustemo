///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer;
use super::calculator01::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(token: Token) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EC1 {
    pub e: Box<E>,
    pub t: T,
}
#[derive(Debug, Clone)]
pub enum E {
    C1(EC1),
    C2(T),
}
pub fn e_c1(e: E, t: T) -> E {
    E::C1(EC1 { e: Box::new(e), t })
}
pub fn e_c2(t: T) -> E {
    E::C2(t)
}
#[derive(Debug, Clone)]
pub struct TC1 {
    pub t: Box<T>,
    pub f: F,
}
#[derive(Debug, Clone)]
pub enum T {
    C1(TC1),
    C2(F),
}
pub fn t_c1(t: T, f: F) -> T {
    T::C1(TC1 { t: Box::new(t), f })
}
pub fn t_c2(f: F) -> T {
    T::C2(f)
}
#[derive(Debug, Clone)]
pub enum F {
    C1(Box<E>),
    C2(Num),
}
pub fn f_c1(e: E) -> F {
    F::C1(Box::new(e))
}
pub fn f_c2(num: Num) -> F {
    F::C2(num)
}
