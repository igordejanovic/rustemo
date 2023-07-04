/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use ::rustemo::context::Context;
use rustemo::lexer;
use super::calculator::{self, TokenKind};
pub type Input = str;
pub type Ctx<'i> = super::calculator::Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Operand = String;
pub fn operand(_ctx: &Ctx, token: Token) -> Operand {
    token.value.into()
}
pub type Operator = String;
pub fn operator(_ctx: &Ctx, token: Token) -> Operator {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub operand_1: Operand,
    pub operator: Operator,
    pub operand_3: Operand,
}
pub fn expression_c1(
    _ctx: &Ctx,
    operand_1: Operand,
    operator: Operator,
    operand_3: Operand,
) -> Expression {
    Expression {
        operand_1,
        operator,
        operand_3,
    }
}
