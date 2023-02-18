use super::custom_lexer_2::{TokenKind, Context};
use super::custom_lexer_2_lexer::Input;
///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type MSBByte = u8;
pub fn msbbyte<'i>(_ctx: &Context<'i>, token: Token<'i>) -> MSBByte {
    token.value[0]
}
pub type NonMSBByte = u8;
pub fn non_msbbyte<'i>(_ctx: &Context<'i>, token: Token<'i>) -> NonMSBByte {
    token.value[0]
}
pub type VarInts = VarInt1;
pub fn var_ints_c1<'i>(_ctx: &Context<'i>, var_int1: VarInt1) -> VarInts {
    var_int1
}
pub type VarInt1 = Vec<VarInt>;
pub fn var_int1_c1<'i>(
    _ctx: &Context<'i>,
    mut var_int1: VarInt1,
    var_int: VarInt,
) -> VarInt1 {
    var_int1.push(var_int);
    var_int1
}
pub fn var_int1_c2<'i>(_ctx: &Context<'i>, var_int: VarInt) -> VarInt1 {
    vec![var_int]
}
/// We are doing a conversion in this action. Other actions are generated.
/// msbbyte0 is an option containing first bytes of the VarInt non_msbbyte
/// contains the last byte
pub type VarInt = i128;
pub fn var_int_c1<'i>(
    _ctx: &Context<'i>,
    msbbyte0: MSBByte0,
    non_msbbyte: NonMSBByte,
) -> VarInt {
    let mut res: i128 = non_msbbyte as i128;
    if let Some(bytes) = msbbyte0 {
        bytes
            .iter()
            .rev()
            .for_each(|b| {
                res <<= 7;
                res |= (b & 0b0111_1111) as i128;
            });
    }
    res
}
pub type MSBByte1 = Vec<MSBByte>;
pub fn msbbyte1_c1<'i>(
    _ctx: &Context<'i>,
    mut msbbyte1: MSBByte1,
    msbbyte: MSBByte,
) -> MSBByte1 {
    msbbyte1.push(msbbyte);
    msbbyte1
}
pub fn msbbyte1_c2<'i>(_ctx: &Context<'i>, msbbyte: MSBByte) -> MSBByte1 {
    vec![msbbyte]
}
pub type MSBByte0 = Option<MSBByte1>;
pub fn msbbyte0_c1<'i>(_ctx: &Context<'i>, msbbyte1: MSBByte1) -> MSBByte0 {
    Some(msbbyte1)
}
pub fn msbbyte0_empty<'i>(_ctx: &Context<'i>) -> MSBByte0 {
    None
}
