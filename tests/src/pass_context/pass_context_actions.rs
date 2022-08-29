///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
use super::pass_context::Context;
use super::Input;
pub type Num = u32;
pub fn num<I: AsRef<Input>>(_context: &Context<I>, token: Token<I>) -> Num {
    token.value.as_ref().parse::<u32>().unwrap() + _context.position as u32
}
pub type A = Num1;
pub fn a_c1<I: AsRef<Input>>(_context: &Context<I>, num1: Num1) -> A {
    num1
}
pub type Num1 = Vec<Num>;
pub fn num1_c1<I: AsRef<Input>>(
    _context: &Context<I>,
    mut num1: Num1,
    num: Num,
) -> Num1 {
    num1.push(num);
    num1
}
pub fn num1_c2<I: AsRef<Input>>(_context: &Context<I>, num: Num) -> Num1 {
    vec![num]
}
