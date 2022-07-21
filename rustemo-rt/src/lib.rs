// See: https://github.com/rust-lang/rfcs/issues/2324
// For local std docs browsing
#[doc(inline)]
pub use std;

#[macro_use]
pub mod common;
pub mod builder;
pub mod grammar;
pub mod index;
pub mod lexer;
pub mod lr;
pub mod parser;
pub mod tree;

pub mod debug;

