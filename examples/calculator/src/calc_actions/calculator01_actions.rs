use super::calculator01::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as BaseToken;
pub type Input = str;
pub type Token<'i> = BaseToken<'i, Input, TokenKind>;
pub type Ctx<'i> = Context<'i, Input>;
pub type Num = f32;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_c1(_ctx: &Ctx, e_1: E, t_3: T) -> E {
    e_1 + t_3
}
pub fn e_t(_ctx: &Ctx, t: T) -> E {
    t
}
pub type T = f32;
pub fn t_c1(_ctx: &Ctx, t_1: T, f_3: F) -> T {
    t_1 * f_3
}
pub fn t_f(_ctx: &Ctx, f: F) -> T {
    f
}
pub type F = f32;
pub fn f_e(_ctx: &Ctx, e: E) -> F {
    e
}
pub fn f_num(_ctx: &Ctx, num: Num) -> F {
    num
}
