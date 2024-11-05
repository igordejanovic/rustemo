use std::fs;

use crate::{grammar::Grammar, output_cmp};

#[test]
fn test_rustemo_grammar() {
    use std::path::PathBuf;

    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src", "lang", "rustemo.rustemo"]
        .iter()
        .collect();
    let grammar: Grammar = fs::read_to_string(path).unwrap().parse().unwrap();

    output_cmp!("src/lang/rustemo.ast", format!("{:#?}", grammar));
}
