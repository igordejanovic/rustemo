use super::json::{self, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as BaseToken;
pub type Input = str;
pub type Ctx<'i> = json::Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = BaseToken<'i, Input, TokenKind>;
pub type JsonNumber = f32;
pub fn json_number(_ctx: &Ctx, token: Token) -> JsonNumber {
    token.value.parse().unwrap()
}
pub type JsonString = String;
pub fn json_string(_ctx: &Ctx, token: Token) -> JsonString {
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
pub fn value_false(_ctx: &Ctx) -> Value {
    Value::False
}
pub fn value_true(_ctx: &Ctx) -> Value {
    Value::True
}
pub fn value_null(_ctx: &Ctx) -> Value {
    Value::Null
}
pub fn value_object(_ctx: &Ctx, object: Object) -> Value {
    Value::Object(Box::new(object))
}
pub fn value_array(_ctx: &Ctx, array: Array) -> Value {
    Value::Array(array)
}
pub fn value_json_number(_ctx: &Ctx, json_number: JsonNumber) -> Value {
    Value::JsonNumber(json_number)
}
pub fn value_json_string(_ctx: &Ctx, json_string: JsonString) -> Value {
    Value::JsonString(json_string)
}
pub type Object = Member0;
pub fn object_member0(_ctx: &Ctx, member0: Member0) -> Object {
    member0
}
pub type Member1 = Vec<Member>;
pub fn member1_c1(_ctx: &Ctx, mut member1: Member1, member: Member) -> Member1 {
    member1.push(member);
    member1
}
pub fn member1_member(_ctx: &Ctx, member: Member) -> Member1 {
    vec![member]
}
pub type Member0 = Option<Member1>;
pub fn member0_member1(_ctx: &Ctx, member1: Member1) -> Member0 {
    Some(member1)
}
pub fn member0_empty(_ctx: &Ctx) -> Member0 {
    None
}
#[derive(Debug, Clone)]
pub struct Member {
    pub json_string: JsonString,
    pub value: Value,
}
pub fn member_c1(_ctx: &Ctx, json_string: JsonString, value: Value) -> Member {
    Member { json_string, value }
}
pub type Array = Value0;
pub fn array_value0(_ctx: &Ctx, value0: Value0) -> Array {
    value0
}
pub type Value1 = Vec<Box<Value>>;
pub fn value1_c1(_ctx: &Ctx, mut value1: Value1, value: Value) -> Value1 {
    value1.push(Box::new(value));
    value1
}
pub fn value1_value(_ctx: &Ctx, value: Value) -> Value1 {
    vec![Box::new(value)]
}
pub type Value0 = Option<Value1>;
pub fn value0_value1(_ctx: &Ctx, value1: Value1) -> Value0 {
    Some(value1)
}
pub fn value0_empty(_ctx: &Ctx) -> Value0 {
    None
}
