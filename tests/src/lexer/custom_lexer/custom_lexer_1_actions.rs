/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::custom_lexer_1::{self, TokenKind};
use super::custom_lexer_1_lexer::Input;
use rustemo::lexer;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Context<'i> = custom_lexer_1::Context<'i, Input>;
pub type VarInt = i128;
/// Here we convert varint slice of u8 to i128
pub fn var_int<'i>(_ctx: &Context<'i>, token: Token<'i>) -> VarInt {
    let mut res: VarInt = 0;
    token
        .value
        .iter()
        .rev()
        .for_each(|b| {
            res <<= 7;
            res |= (b & 0b0111_1111) as i128;
        });
    res
}
pub type VarInts = VarInt1;
pub fn var_ints_var_int1(_ctx: &Context, var_int1: VarInt1) -> VarInts {
    var_int1
}
pub type VarInt1 = Vec<VarInt>;
pub fn var_int1_c1(_ctx: &Context, mut var_int1: VarInt1, var_int: VarInt) -> VarInt1 {
    var_int1.push(var_int);
    var_int1
}
pub fn var_int1_var_int(_ctx: &Context, var_int: VarInt) -> VarInt1 {
    vec![var_int]
}
