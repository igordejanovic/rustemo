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
