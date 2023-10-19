use super::calc::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as BaseToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = BaseToken<'i, Input, TokenKind>;
pub type Num = String;
pub fn num(_ctx: &Ctx, token: Token) -> Num {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct EC1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct EC2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    C1(EC1),
    C2(EC2),
    Num(Num),
}
pub fn e_c1(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::C1(EC1 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_c2(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::C2(EC2 {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_num(_ctx: &Ctx, num: Num) -> E {
    E::Num(num)
}
