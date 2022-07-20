pub mod generator;
pub mod build;
pub mod grammar;
pub mod parser;
pub mod settings;

#[rustfmt::skip]
mod rustemo;
#[rustfmt::skip]
mod rustemo_types;

mod rustemo_actions;
mod table;
pub(crate) mod tests;

pub use crate::build::generate_parsers;
