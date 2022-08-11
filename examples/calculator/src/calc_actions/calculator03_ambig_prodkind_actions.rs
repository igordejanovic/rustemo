///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = f32;
pub fn num<'a>(token: Token<&'a str>) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_add(e_1: E, e_3: E) -> E {
    e_1 + e_3
}
pub fn e_sub(e_1: E, e_3: E) -> E {
    e_1 - e_3
}
pub fn e_mul(e_1: E, e_3: E) -> E {
    e_1 * e_3
}
pub fn e_div(e_1: E, e_3: E) -> E {
    e_1 / e_3
}
pub fn e_pow(e_1: E, e_3: E) -> E {
    f32::powf(e_1, e_3)
}
pub fn e_paren(e: E) -> E {
    e
}
pub fn e_num(num: Num) -> E {
    num
}