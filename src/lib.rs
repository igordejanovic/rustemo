// See: https://github.com/rust-lang/rfcs/issues/2324
// For local std docs browsing
#[doc(inline)]
pub use std;

mod builder;
mod common;
pub mod grammar;
mod lang;
mod lexer;
mod lr;
pub mod parser;
pub mod tree;

pub(crate) mod debug;
pub(crate) mod tests;
