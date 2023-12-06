//! This crate is the runtime for the generated Rustemo parsers.
// See: https://github.com/rust-lang/rfcs/issues/2324
// For local std docs browsing
// #[doc(inline)]
// pub use std;

#[macro_use]
mod common;
#[macro_use]
pub mod debug;

mod builder;
mod context;
mod error;
mod input;
mod lexer;
mod location;
mod parser;
mod utils;

mod lr;
//#[cfg(feature = "glr")]
mod glr;

// Public API
pub use crate::context::Context;
pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::input::Input;
pub use crate::location::{LineColumn, Location, Position, ValLoc};

pub use crate::builder::Builder;
pub use crate::lexer::{Lexer, StringLexer, Token, TokenRecognizer};
pub use crate::lr::{
    builder::{LRBuilder, SliceBuilder, TreeBuilder, TreeNode},
    context::LRContext,
    parser::{Action, LRParser, ParserDefinition},
};
pub use crate::parser::{Parser, State};

//#[cfg(feature = "glr")]
pub use crate::glr::{
    gss::{Forest, GssHead},
    parser::GlrParser,
};
