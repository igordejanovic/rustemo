/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::calculator04_ambig_lhs::{Context, TokenKind};
use rustemo::lexer;
pub type Token<'i> = lexer::Token<'i, str, TokenKind>;
pub type Num = f32;
pub fn num(_ctx: &Context, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
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
pub fn e_pow(_ctx: &Context, base: E, exp: E) -> E {
    f32::powf(base, exp)
}
pub fn e_paren(_ctx: &Context, e: E) -> E {
    e
}
pub fn e_num(_ctx: &Context, num: Num) -> E {
    num
}
