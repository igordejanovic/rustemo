//! This crate provides a Rustemo grammars compiler as a CLI command, and an API
//! for usage from `build.rs` scripts.
//!
//! When this crate is installed `rcomp` command is available that can be run
//! over Rustemo grammars to produce parsers.
//!
//! The entry point into API is [Settings::new] which provide a default settings
//! value which can be further configured in a builder pattern style calling
//! methods of [Settings] and finally executed using [Settings::process_dir] or
//! [Settings::process_grammar] methods.
//!
//! ## Example
//!
//! ```rust,ignore
//! rustemo_compiler::Settings::new().force(true).in_source_tree().process_dir()
//! ```
//!
//! # Processing grammars
//!
//! For default settings there are [process_crate_dir], [process_dir] and
//! [process_grammar] shortcut functions.
//!
//! ## Example
//!
//! Usual pattern you can use in `build.rs` is:
//!
//! ```rust,ignore
//! if let Err(e) = rustemo_compiler::process_crate_dir() {
//!     eprintln!("{}", e);
//!     exit(1);
//! }
//! ```
#![forbid(unsafe_code)]
#[macro_use]
extern crate rustemo;

pub mod grammar;
pub mod utils;

pub use crate::settings::{
    process_crate_dir, process_dir, process_grammar, BuilderType, GeneratorTableType, LexerType,
    ParserAlgo, Settings,
};
pub use crate::table::TableType;

pub use crate::error::Error;
pub use crate::error::Result;

// For output_cmp macro
pub use crate::utils::string_difference;

mod error;
mod generator;
mod index;
mod lang;
mod settings;
mod table;
