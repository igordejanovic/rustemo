use std::path::PathBuf;

use super::generate_parser;

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

// #[test]
// fn test_rustemo_bootstrap() {
//     let mut grammar_file = PathBuf::from(file!());
//     grammar_file.pop();
//     grammar_file.push("rustemo");
//     grammar_file.push("rustemo.rustemo");

//     generate_parser(grammar_file).unwrap();

// }
