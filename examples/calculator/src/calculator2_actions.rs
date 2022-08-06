use rustemo_rt::lexer::Token;
pub type Num = f32;
pub fn num<'i>(token: Token<&'i str>) -> Num {
    token.value.parse().unwrap()
}
pub type E = f32;
pub fn e_1(left: E, right: E) -> E {
    left + right
}
pub fn e_2(left: E, right: E) -> E {
    left - right
}
pub fn e_3(left: E, right: E) -> E {
    left * right
}
pub fn e_4(left: E, right: E) -> E {
    left / right
}
pub fn e_5(left: E, right: E) -> E {
    f32::powf(left, right)
}
pub fn e_6(e: E) -> E {
    e
}
pub fn e_7(num: E) -> E {
    num
}
#[derive(Debug, Clone)]
pub struct E1 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct E2 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct E3 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct E4 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct E5 {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
