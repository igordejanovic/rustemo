#![allow(unused_imports)]
#![cfg(test)]
mod ambiguity;
mod builder;
mod errors;
mod layout;
mod lexer;
mod partial;
mod rule_patterns;
mod sugar;

/// Uses in tests to calculate local file path.
/// Requires call to file!() as a first parameter.
#[macro_export]
macro_rules! local_file {
    ($this:expr, $local_path:expr) => {
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join($this)
            .with_file_name($local_path)
    };
}
