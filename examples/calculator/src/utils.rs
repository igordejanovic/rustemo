// FIXME: Use these stuff from rustemo crate
pub(crate) fn string_difference(
    a: &String,
    b: &String,
) -> Option<(usize, (char, char))> {
    Some(
        a.chars()
            .zip(b.chars())
            .enumerate()
            .find(|(_, (a, b))| a != b)?,
    )
}

#[macro_export]
macro_rules! output_cmp {
    ($path:expr, $out_str:expr) => {{
        use {
            crate::utils::string_difference,
            std::{fs, path::PathBuf},
        };
        let t_path: PathBuf =
            [env!("CARGO_MANIFEST_DIR"), $path].iter().collect();

        if t_path.exists() {
            let content: String = fs::read_to_string(&t_path)
                .unwrap_or_else(|err| {
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
