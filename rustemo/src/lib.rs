#[macro_use]
extern crate rustemo_rt;

pub mod build;
pub mod generator;
pub mod grammar;
pub mod settings;
pub mod error;

mod lang;
mod table;
pub(crate) mod tests;

pub use crate::build::generate_parsers;
pub use crate::generator::generate_parser;

pub use crate::error::Result;
pub use crate::error::Error;
