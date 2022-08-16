///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = f32;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_c1(e_1: E, t_3: T) -> E {
    e_1 + t_3
}
pub fn e_c2(t: T) -> E {
    t
}
pub type T = f32;
pub fn t_c1(t_1: T, f_3: F) -> T {
    t_1 * f_3
}
pub fn t_c2(f: F) -> T {
    f
}
pub type F = f32;
pub fn f_c1(e: E) -> F {
    e
}
pub fn f_c2(num: Num) -> F {
    num
}
