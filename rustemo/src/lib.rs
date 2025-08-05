//! This crate is the runtime for the generated Rustemo parsers.
// See: https://github.com/rust-lang/rfcs/issues/2324
// For local std docs browsing
// #[doc(inline)]
// pub use std;
#![forbid(unsafe_code)]
#[macro_use]
mod common;
#[macro_use]
pub mod debug;

mod builder;
mod context;
mod error;
mod input;
mod lexer;
mod parser;
mod position;
mod utils;

mod lr;
//#[cfg(feature = "glr")]
mod glr;

// Public API
pub use crate::context::Context;
pub use crate::error::Result;
pub use crate::error::{Error, ParseError};
pub use crate::input::Input;
pub use crate::position::{LineColumn, Position, SourceSpan, ValSpan};

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

// Reexporting dependencies of generated parsers so that users of the library
// do not have to pollute their Cargo.toml.
// See: https://github.com/igordejanovic/rustemo/issues/15
pub use yansi;
pub use fancy_regex;
pub use once_cell;
pub use regex;
