use rustemo_tools::output_cmp;
use rustemo::rustemo_mod;
use std::fs;
use std::path::PathBuf;

use self::custom_lexer_1::CustomLexer1Parser;
use self::custom_lexer_2::CustomLexer2Parser;

mod custom_lexer_1_lexer;
mod custom_lexer_2_lexer;

rustemo_mod!(custom_lexer_1, "/src/custom_lexer");
mod custom_lexer_1_actions;
rustemo_mod!(custom_lexer_2, "/src/custom_lexer");
mod custom_lexer_2_actions;

#[test]
fn custom_lexer_1() {
    let bytes_file = &[
        env!("CARGO_MANIFEST_DIR"),
        "src/custom_lexer/custom_lexer.bytes",
    ]
    .iter()
    .collect::<PathBuf>();
    let bytes = std::fs::read(bytes_file).unwrap();
    let result = CustomLexer1Parser::parse(&*bytes);
    output_cmp!(
        "src/custom_lexer/custom_lexer_1.ast",
        format!("{:#?}", result)
    );
}

#[test]
fn custom_lexer_2() {
    let bytes_file = &[
        env!("CARGO_MANIFEST_DIR"),
        "src/custom_lexer/custom_lexer.bytes",
    ]
    .iter()
    .collect::<PathBuf>();
    let bytes = std::fs::read(bytes_file).unwrap();
    let result = CustomLexer2Parser::parse(&*bytes);
    output_cmp!(
        "src/custom_lexer/custom_lexer_2.ast",
        format!("{:#?}", result)
    );
}
