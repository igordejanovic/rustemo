//! This crate provides a Rustemo grammars compiler as a CLI command, and an API
//! for usage from `build.rs` scripts.
//!
//! When this crate is installed `rustemo` command is available that can be run
//! over Rustemo grammars to produce parsers.
//!
//! The entry point into API is [Settings::new] which provide a default settings
//! value which can be further configured in a builder pattern style and
//! executed using [Settings::process_dir] or [Settings::process_grammar]
//! methods.
//!
//! For a default settings there are [process_dir] and [process_grammar]
//! shortcut functions.
#[macro_use]
extern crate rustemo;

pub mod grammar;
pub mod utils;

pub use crate::settings::{
    process_crate_dir, process_dir, process_grammar, BuilderType, LexerType,
    ParserAlgo, Settings,
};
pub use crate::table::TableType;

pub use crate::error::Error;
pub use crate::error::Result;

// For output_cmp macro
pub use crate::utils::string_difference;

mod error;
mod generator;
mod lang;
mod settings;
mod table;
