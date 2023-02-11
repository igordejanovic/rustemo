///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::calculator::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
/// ANCHOR: number
pub type Number = f32;
/// ANCHOR_END: number
/// ANCHOR: number_action
pub fn number(token: Token) -> Number {
    token.value.parse().unwrap()
}
/// ANCHOR_END: number_action
/// ANCHOR: expression
pub type E = f32;
/// ANCHOR_END: expression
/// ANCHOR: actions
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
pub fn e_c5(number: Number) -> E {
    number
}
/// ANCHOR_END: actions
