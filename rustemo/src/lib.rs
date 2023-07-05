// See: https://github.com/rust-lang/rfcs/issues/2324
// For local std docs browsing
#[doc(inline)]
pub use std;

#[macro_use]
pub mod common;
pub mod builder;
pub mod context;
pub mod error;
pub mod input;
pub mod lexer;
pub mod location;
pub mod parser;

pub mod debug;
pub mod lr;

pub use crate::error::Error;
pub use crate::error::Result;
