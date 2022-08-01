#[macro_use]
extern crate rustemo_rt;

pub mod build;
pub mod generator;
pub mod grammar;
pub mod settings;

#[rustfmt::skip]
#[cfg(not(feature = "bootstrap"))]
mod rustemo;

// In bootstrapping mode use the generated parser from the OUT_DIR folder
#[rustfmt::skip]
#[cfg(feature = "bootstrap")]
rustemo_mod!(rustemo, "/src/rustemo.rs");

mod rustemo_actions;
mod table;
pub(crate) mod tests;

pub use crate::build::generate_parsers;
pub use crate::generator::generate_parser;
