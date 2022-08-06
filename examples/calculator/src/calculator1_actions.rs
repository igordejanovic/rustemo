use rustemo_rt::lexer::Token;
pub type Num = i32;
pub fn num<'i>(token: Token<&'i str>) -> Num {
    token.value.parse().unwrap()
}
pub type E = i32;
pub fn e_1(left: E, right: T) -> E {
    left + right
}
pub fn e_2(t: T) -> E {
    t
}
pub type T = i32;
pub fn t_1(left: T, right: F) -> T {
    left * right
}
pub fn t_2(f: F) -> T {
    f
}
pub type F = i32;
pub fn f_1(e: E) -> F {
    e
}
pub fn f_2(n: Num) -> F {
    n
}
#[derive(Debug, Clone)]
pub struct E1 {
    pub e_1: Box<E>,
    pub t_3: T,
}
#[derive(Debug, Clone)]
pub struct T1 {
    pub t_1: Box<T>,
    pub f_3: F,
}
