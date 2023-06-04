/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::calculator01::Context;
use super::calculator01::TokenKind;
use rustemo::lexer;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(_ctx: &Context, token: Token) -> Num {
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
    T(T),
}
pub fn e_c1(_ctx: &Context, e: E, t: T) -> E {
    E::C1(EC1 { e: Box::new(e), t })
}
pub fn e_t(_ctx: &Context, t: T) -> E {
    E::T(t)
}
#[derive(Debug, Clone)]
pub struct TC1 {
    pub t: Box<T>,
    pub f: F,
}
#[derive(Debug, Clone)]
pub enum T {
    C1(TC1),
    F(F),
}
pub fn t_c1(_ctx: &Context, t: T, f: F) -> T {
    T::C1(TC1 { t: Box::new(t), f })
}
pub fn t_f(_ctx: &Context, f: F) -> T {
    T::F(f)
}
#[derive(Debug, Clone)]
pub enum F {
    E(Box<E>),
    Num(Num),
}
pub fn f_e(_ctx: &Context, e: E) -> F {
    F::E(Box::new(e))
}
pub fn f_num(_ctx: &Context, num: Num) -> F {
    F::Num(num)
}
