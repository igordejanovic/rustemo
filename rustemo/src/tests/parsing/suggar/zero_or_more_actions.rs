///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
#[derive(Debug, Clone)]
pub enum A {
    V1(Option<B>),
}
pub fn a_v1(b: Option<B>) -> A {
    A::V1(b)
}
#[derive(Debug, Clone)]
pub enum B {
    V1,
    Empty,
}
pub fn b_v1() -> Option<B> {
    Some(B::V1)
}
pub fn b_empty() -> Option<B> {
    Some(B::Empty)
}
