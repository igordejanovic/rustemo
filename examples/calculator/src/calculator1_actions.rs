use rustemo_rt::lexer::Token;

pub type Num = i32;
pub fn num<'i>(token: Token<&'i str>) -> Num {
    token.value.parse().unwrap()
}

pub type E = i32;
pub fn e_p0(left: E, right: T) -> E {
    left + right
}
pub fn e_p1(t: T) -> E {
    t
}

pub type T = i32;
pub fn t_p0(left: T, right: F) -> T {
    left * right
}
pub fn t_p1(f: F) -> T {
   f
}

pub type F = i32;
pub fn f_p0(e: E) -> F {
    e
}
pub fn f_p1(n: Num) -> F {
    n
}
