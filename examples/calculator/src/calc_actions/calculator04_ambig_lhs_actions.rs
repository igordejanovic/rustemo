use super::calculator04_ambig_lhs::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as BaseToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
pub type Token<'i> = BaseToken<'i, Input, TokenKind>;
pub type Num = f32;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
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
pub fn e_pow(_ctx: &Ctx, base: E, exp: E) -> E {
    f32::powf(base, exp)
}
pub fn e_paren(_ctx: &Ctx, e: E) -> E {
    e
}
pub fn e_num(_ctx: &Ctx, num: Num) -> E {
    num
}
