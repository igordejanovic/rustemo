use super::calclr::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type E = i32;
pub type Number = i32;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.parse().unwrap()
}
pub fn e_c1(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 + e_3
}
pub fn e_c2(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    e_1 * e_3
}
pub fn e_number(_ctx: &Ctx, number: Number) -> E {
    number
}
