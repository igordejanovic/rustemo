use super::calc_eval::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as BaseToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = BaseToken<'i, Input, TokenKind>;
pub type Num = f32;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_add(_ctx: &Ctx, left: E, right: E) -> E {
    left + right
}
pub fn e_mul(_ctx: &Ctx, left: E, right: E) -> E {
    left * right
}
pub fn e_num(_ctx: &Ctx, num: Num) -> E {
    num
}
