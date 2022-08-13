#[macro_use]
extern crate rustemo_rt;

pub mod build;
pub mod error;
pub mod generator;
pub mod grammar;
pub mod settings;

mod lang;
mod table;

pub mod utils;

pub use crate::build::generate_parsers;
pub use crate::generator::generate_parser;

pub use crate::error::Error;
pub use crate::error::Result;

// For output_cmp macro
pub use crate::utils::string_difference;
