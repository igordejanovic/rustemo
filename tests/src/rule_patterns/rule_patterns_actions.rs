///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
pub type A = Option<ANE>;
#[derive(Debug, Clone)]
pub enum ANE {
    V1,
}
pub fn a_v1() -> A {
    Some(ANE::V1)
}
pub fn a_empty() -> A {
    None
}
#[derive(Debug, Clone)]
pub struct BV1 {
    pub b: Box<B>,
    pub a: A,
}
pub type B = Option<BNE>;
#[derive(Debug, Clone)]
pub enum BNE {
    V1(BV1),
    V2(A),
}
pub fn b_v1(b: B, a: A) -> B {
    Some(BNE::V1(BV1 { b: Box::new(b), a }))
}
pub fn b_v2(a: A) -> B {
    Some(BNE::V2(a))
}
pub fn b_empty() -> B {
    None
}
#[derive(Debug, Clone)]
pub struct CV1 {
    pub c: Box<C>,
    pub a: A,
}
#[derive(Debug, Clone)]
pub enum C {
    V1(CV1),
    V2(A),
}
pub fn c_v1(c: C, a: A) -> C {
    C::V1(CV1 { c: Box::new(c), a })
}
pub fn c_v2(a: A) -> C {
    C::V2(a)
}
pub type D = Option<A>;
pub fn d_v1(a: A) -> D {
    Some(a)
}
pub fn d_empty() -> D {
    None
}
#[derive(Debug, Clone)]
pub struct EV1 {
    pub a: A,
    pub b: B,
    pub c: C,
}
pub type E = Option<ENE>;
#[derive(Debug, Clone)]
pub enum ENE {
    V1(EV1),
    V2(D),
}
pub fn e_v1(a: A, b: B, c: C) -> E {
    Some(ENE::V1(EV1 { a, b, c }))
}
pub fn e_v2(d: D) -> E {
    Some(ENE::V2(d))
}
pub fn e_empty() -> E {
    None
}
#[derive(Debug, Clone)]
pub struct FV1 {
    pub b: B,
    pub f: Box<F>,
}
pub type F = Option<FNE>;
#[derive(Debug, Clone)]
pub enum FNE {
    V1(FV1),
    V2(B),
}
pub fn f_v1(b: B, f: F) -> F {
    Some(FNE::V1(FV1 { b, f: Box::new(f) }))
}
pub fn f_v2(b: B) -> F {
    Some(FNE::V2(b))
}
pub fn f_empty() -> F {
    None
}
