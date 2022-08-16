pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn string_difference(a: &str, b: &str) -> Option<(usize, (char, char))> {
    Some(
        a.chars()
            .zip(b.chars())
            .enumerate()
            .find(|(_, (a, b))| a != b)?,
    )
}

/// Used in tests for storing and comparing string representations in files.
///
/// # Example
///
/// ```rust
/// let states = lr_states_for_grammar(&grammar, &settings);
/// output_cmp!("grammar.expected.txt", format!("{states:#?}"));
/// ```
///
/// If the file `grammar.expected.txt` exists its content will be compared
/// to the string of the second parameter. If the file doesn't exist it will
/// be created with the content of the second parameter. The idea is to
/// check manually the content first time it is created and commit to git repo.
/// Whenever the file is changed the assert will fail. In that case we delete
/// output file, run the test to produce the new version and investigate the
/// diff with git. If everything is expected we commit the new version of the
/// output file.
///
/// This is helpful for testing the content of larger structures.
#[macro_export]
macro_rules! output_cmp {
    ($path:expr, $out_str:expr) => {{
        use {
            crate::string_difference,
            std::{fs, path::PathBuf},
        };
        let t_path: PathBuf =
            [env!("CARGO_MANIFEST_DIR"), $path].iter().collect();

        if t_path.exists() {
            let content: String =
                fs::read_to_string(&t_path).unwrap_or_else(|err| {
                    panic!("Cannot load output file {:?}: {}", t_path, err)
                });
            if let Some(diff) = string_difference(&content, &$out_str) {
                assert!(false, "Strings differ at: {:?}", diff)
            }
        } else {
            fs::write(&t_path, $out_str).unwrap_or_else(|err| {
                panic!("Error writing file {:?}: {}", t_path, err)
            });
        }
    }};
}
pub use output_cmp;
