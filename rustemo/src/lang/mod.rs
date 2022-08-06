#[rustfmt::skip]
#[cfg(not(feature = "bootstrap"))]
pub(crate) mod rustemo;

// In bootstrapping mode use the generated parser from the OUT_DIR folder
#[rustfmt::skip]
#[cfg(feature = "bootstrap")]
rustemo_mod!(rustemo, "/src/rustemo.rs");

pub(crate) mod rustemo_actions;

#[cfg(test)]
mod tests;
