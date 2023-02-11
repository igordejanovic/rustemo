use super::calculator04_ambig_lhs::TokenKind;
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Token<'i> = lexer::Token<'i, str, TokenKind>;
pub type Num = f32;
pub fn num(token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_add(left: E, right: E) -> E {
    left + right
}
pub fn e_sub(left: E, right: E) -> E {
    left - right
}
pub fn e_mul(left: E, right: E) -> E {
    left * right
}
pub fn e_div(left: E, right: E) -> E {
    left / right
}
pub fn e_pow(base: E, exp: E) -> E {
    f32::powf(base, exp)
}
pub fn e_paren(e: E) -> E {
    e
}
pub fn e_num(num: Num) -> E {
    num
}
