use super::calculator03_ambig_prodkind::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Num = f32;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_add(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 + e_3
}
pub fn e_sub(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 - e_3
}
pub fn e_mul(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 * e_3
}
pub fn e_div(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 / e_3
}
pub fn e_pow(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    f32::powf(e_1, e_3)
}
pub fn e_paren(_ctx: &Ctx, e: E) -> E {
    e
}
pub fn e_num(_ctx: &Ctx, num: Num) -> E {
    num
}
