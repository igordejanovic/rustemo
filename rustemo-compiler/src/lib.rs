//! This crate provides a Rustemo grammars compiler as a CLI command, and an API
//! for usage from `build.rs` scripts.
//!
//! When this crate is installed `rustemo` command is available that can be run
//! over Rustemo grammars to produce parsers.
//!
//! The entry point into API if non-default setting is needed is [with_settings]
//! which returns a [RustemoSettings] value which can be further configured in a
//! builder pattern style.
//!
//! For a default settings there are [process_dir] and [process_grammar] functions.
#[macro_use]
extern crate rustemo;

pub mod grammar;
pub mod utils;

pub use crate::api::{
    process_dir, process_grammar, with_settings, RustemoSettings,
};
pub use crate::settings::{BuilderType, LexerType, ParserAlgo};
pub use crate::table::TableType;

pub use crate::error::Error;
pub use crate::error::Result;

// For output_cmp macro
pub use crate::utils::string_difference;

mod api;
mod error;
mod generator;
mod lang;
mod settings;
mod table;
