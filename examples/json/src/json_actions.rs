/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use super::json::Context;
use super::json::TokenKind;
use rustemo::lexer;
pub type Input = str;
#[allow(dead_code)]
pub type Token<'i> = lexer::Token<'i, Input, TokenKind>;
pub type JsonNumber = f32;
pub fn json_number<'i>(_ctx: &Context<'i>, token: Token<'i>) -> JsonNumber {
    token.value.parse().unwrap()
}
pub type JsonString = String;
pub fn json_string<'i>(_ctx: &Context<'i>, token: Token<'i>) -> JsonString {
    token.value.into()
}
#[derive(Debug, Clone)]
pub enum Value {
    False,
    True,
    Null,
    Object(Box<Object>),
    Array(Array),
    JsonNumber(JsonNumber),
    JsonString(JsonString),
}
pub fn value_false(_ctx: &Context) -> Value {
    Value::False
}
pub fn value_true(_ctx: &Context) -> Value {
    Value::True
}
pub fn value_null(_ctx: &Context) -> Value {
    Value::Null
}
pub fn value_object(_ctx: &Context, object: Object) -> Value {
    Value::Object(Box::new(object))
}
pub fn value_array(_ctx: &Context, array: Array) -> Value {
    Value::Array(array)
}
pub fn value_json_number(_ctx: &Context, json_number: JsonNumber) -> Value {
    Value::JsonNumber(json_number)
}
pub fn value_json_string(_ctx: &Context, json_string: JsonString) -> Value {
    Value::JsonString(json_string)
}
pub type Object = Member0;
pub fn object_member0(_ctx: &Context, member0: Member0) -> Object {
    member0
}
pub type Member1 = Vec<Member>;
pub fn member1_c1(
    _ctx: &Context,
    mut member1: Member1,
    member: Member,
) -> Member1 {
    member1.push(member);
    member1
}
pub fn member1_member(_ctx: &Context, member: Member) -> Member1 {
    vec![member]
}
pub type Member0 = Option<Member1>;
pub fn member0_member1(_ctx: &Context, member1: Member1) -> Member0 {
    Some(member1)
}
pub fn member0_empty(_ctx: &Context) -> Member0 {
    None
}
#[derive(Debug, Clone)]
pub struct Member {
    pub json_string: JsonString,
    pub value: Value,
}
pub fn member_c1(
    _ctx: &Context,
    json_string: JsonString,
    value: Value,
) -> Member {
    Member { json_string, value }
}
pub type Array = Value0;
pub fn array_value0(_ctx: &Context, value0: Value0) -> Array {
    value0
}
pub type Value1 = Vec<Box<Value>>;
pub fn value1_c1(_ctx: &Context, mut value1: Value1, value: Value) -> Value1 {
    value1.push(Box::new(value));
    value1
}
pub fn value1_value(_ctx: &Context, value: Value) -> Value1 {
    vec![Box::new(value)]
}
pub type Value0 = Option<Value1>;
pub fn value0_value1(_ctx: &Context, value1: Value1) -> Value0 {
    Some(value1)
}
pub fn value0_empty(_ctx: &Context) -> Value0 {
    None
}
