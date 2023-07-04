/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use ::rustemo::context::Context;
use rustemo::lexer;
use super::calculator::{self, TokenKind};
pub type Input = str;
pub type Ctx<'i> = super::calculator::Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
/// ANCHOR: number
pub type Number = f32;
/// ANCHOR_END: number
/// ANCHOR: number_action
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.parse().unwrap()
}
/// ANCHOR_END: number_action
/// ANCHOR: expression
pub type E = f32;
/// ANCHOR_END: expression
/// ANCHOR: actions
pub fn e_add(_ctx: &Ctx, left: E, right: E) -> E {
    left + right
}
pub fn e_sub(_ctx: &Ctx, left: E, right: E) -> E {
    left - right
}
pub fn e_mul(_ctx: &Ctx, left: E, right: E) -> E {
    left * right
}
pub fn e_div(_ctx: &Ctx, left: E, right: E) -> E {
    left / right
}
pub fn e_number(_ctx: &Ctx, number: Number) -> E {
    number
}
/// ANCHOR_END: actions
#[allow(dead_code)]
type Dummy = u32;
