#[macro_use]
extern crate rustemo;

pub mod api;
pub mod error;
pub mod generator;
pub mod grammar;

mod lang;
mod table;

pub mod utils;

pub use crate::api::{process_dir, process_grammar, with_settings};

pub use crate::error::Error;
pub use crate::error::Result;

// For output_cmp macro
pub use crate::utils::string_difference;
