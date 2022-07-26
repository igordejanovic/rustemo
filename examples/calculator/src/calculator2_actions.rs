use rustemo_rt::lexer::Token;

pub type Num = f32;
pub fn num<'i>(token: Token<&'i str>) -> Num {
    token.value.parse().unwrap()
}

pub type E = f32;
pub fn e_p0(left: E, right: E) -> E { left + right }
pub fn e_p1(left: E, right: E) -> E { left - right }
pub fn e_p2(left: E, right: E) -> E { left * right }
pub fn e_p3(left: E, right: E) -> E { left / right }
pub fn e_p4(left: E, right: E) -> E { f32::powf(left, right) }
pub fn e_p5(e: E) -> E { e }
pub fn e_p6(num: E) -> E { num }
