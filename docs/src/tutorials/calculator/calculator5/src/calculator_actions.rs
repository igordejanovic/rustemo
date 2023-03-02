use super::calculator::{Context, TokenKind};
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
/// ANCHOR: number
pub type Number = f32;
/// ANCHOR_END: number
/// ANCHOR: number_action
pub fn number<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Number {
    token.value.parse().unwrap()
}
/// ANCHOR_END: number_action
/// ANCHOR: expression
pub type E = f32;
/// ANCHOR_END: expression
/// ANCHOR: actions
pub fn e_add(_ctx: &Context, left: E, right: E) -> E {
    left + right
}
pub fn e_sub(_ctx: &Context, left: E, right: E) -> E {
    left - right
}
pub fn e_mul(_ctx: &Context, left: E, right: E) -> E {
    left * right
}
pub fn e_div(_ctx: &Context, left: E, right: E) -> E {
    left / right
}
pub fn e_c5(_ctx: &Context, number: Number) -> E {
    number
}
/// ANCHOR_END: actions
#[allow(dead_code)]
type Dummy = u32;
