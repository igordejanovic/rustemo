///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
#[derive(Debug, Clone)]
pub enum A {
    V1(B),
}
pub fn a_v1(b: B) -> A {
    A::V1(b)
}
#[derive(Debug, Clone)]
pub enum TC1 {
    V1(TC1),
    V2,
}
pub fn tc1_v1(tc1: TC1) -> TC1 {
    TC1::V1(tc1)
}
pub fn tc1_v2() -> TC1 {
    TC1::V2
}
#[derive(Debug, Clone)]
pub enum TC0 {
    V1,
    V2(EMPTY),
}
pub fn tc0_v1() -> TC0 {
    TC0::V1
}
pub fn tc0_v2(empty: EMPTY) -> TC0 {
    TC0::V2(empty)
}
pub type B = Option<BNE>;
#[derive(Debug, Clone)]
pub enum BNE {
    V1,
}
pub fn b_v1() -> B {
    Some(BNE::V1)
}
pub fn b_empty() -> B {
    None
}
