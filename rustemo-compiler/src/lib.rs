#[macro_use]
extern crate rustemo;

pub mod grammar;
pub mod utils;

pub use crate::api::{
    process_dir, process_grammar, with_settings, RustemoSettings,
};
pub use crate::settings::{BuilderType, LexerType, ParserAlgo, Settings};
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
