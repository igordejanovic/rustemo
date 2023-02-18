///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use super::calculator02_ambig::{Context, TokenKind};
use rustemo::lexer;
pub type Token<'i> = lexer::Token<'i, str, TokenKind>;
pub type Num = f32;
pub fn num(_ctx: &Context, token: Token) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_c1(_ctx: &Context, e_1: E, e_3: E) -> E {
    e_1 + e_3
}
pub fn e_c2(_ctx: &Context, e_1: E, e_3: E) -> E {
    e_1 - e_3
}
pub fn e_c3(_ctx: &Context, e_1: E, e_3: E) -> E {
    e_1 * e_3
}
pub fn e_c4(_ctx: &Context, e_1: E, e_3: E) -> E {
    e_1 / e_3
}
pub fn e_c5(_ctx: &Context, e_1: E, e_3: E) -> E {
    f32::powf(e_1, e_3)
}
pub fn e_c6(_ctx: &Context, e: E) -> E {
    e
}
pub fn e_num(_ctx: &Context, num: Num) -> E {
    num
}
