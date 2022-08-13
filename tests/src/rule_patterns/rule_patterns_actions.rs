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
pub type B = Vec<A>;
pub fn b_v1(mut b: B, a: A) -> B {
    b.push(a);
    b
}
pub fn b_v2(a: A) -> B {
    vec![a]
}
pub fn b_empty() -> B {
    vec![]
}
pub type C = Vec<A>;
pub fn c_v1(mut c: C, a: A) -> C {
    c.push(a);
    c
}
pub fn c_v2(a: A) -> C {
    vec![a]
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
pub type F = Vec<B>;
pub fn f_v1(b: B, mut f: F) -> F {
    f.push(b);
    f
}
pub fn f_v2(b: B) -> F {
    vec![b]
}
pub fn f_empty() -> F {
    vec![]
}
