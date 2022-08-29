///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type Num = f32;
pub fn num<I: AsRef<super::Input>>(token: Token<I>) -> Num {
    token.value.as_ref().parse().unwrap()
}
pub type E = f32;
pub fn e_c1(e_1: E, e_3: E) -> E {
    e_1 + e_3
}
pub fn e_c2(e_1: E, e_3: E) -> E {
    e_1 - e_3
}
pub fn e_c3(e_1: E, e_3: E) -> E {
    e_1 * e_3
}
pub fn e_c4(e_1: E, e_3: E) -> E {
    e_1 / e_3
}
pub fn e_c5(e_1: E, e_3: E) -> E {
    f32::powf(e_1, e_3)
}
pub fn e_c6(e: E) -> E {
    e
}
pub fn e_num(num: Num) -> E {
    num
}
