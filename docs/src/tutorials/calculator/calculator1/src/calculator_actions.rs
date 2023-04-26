///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo::lexer;
use super::calculator::Context;
use super::calculator::TokenKind;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type Operand = String;
pub fn operand<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Operand {
    token.value.into()
}
pub type Operator = String;
pub fn operator<'i>(_ctx: &Context<'i>, token: Token<'i>) -> Operator {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub operand_1: Operand,
    pub operator: Operator,
    pub operand_3: Operand,
}
pub fn expression_c1(
    _ctx: &Context,
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
