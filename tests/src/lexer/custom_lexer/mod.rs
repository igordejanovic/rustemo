use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;
use std::fs;
use std::path::PathBuf;

use self::custom_lexer_1::CustomLexer1Parser;
use self::custom_lexer_1_lexer::MyCustomLexer1;
use self::custom_lexer_2::CustomLexer2Parser;
use self::custom_lexer_2_lexer::MyCustomLexer2;

mod custom_lexer_1_lexer;
mod custom_lexer_2_lexer;

rustemo_mod!(custom_lexer_1, "/src/lexer/custom_lexer");
#[rustfmt::skip]
mod custom_lexer_1_actions;
rustemo_mod!(custom_lexer_2, "/src/lexer/custom_lexer");
#[rustfmt::skip]
mod custom_lexer_2_actions;

#[test]
fn custom_lexer_1() {
    let bytes_file = &[
        env!("CARGO_MANIFEST_DIR"),
        "src/lexer/custom_lexer/custom_lexer.bytes",
    ]
    .iter()
    .collect::<PathBuf>();
    let bytes = std::fs::read(bytes_file).unwrap();
    let result = CustomLexer1Parser::new(MyCustomLexer1::new()).parse(&bytes);
    output_cmp!(
        "src/lexer/custom_lexer/custom_lexer_1.ast",
        format!("{:#?}", result)
    );
}

#[test]
fn custom_lexer_2() {
    let bytes_file = &[
        env!("CARGO_MANIFEST_DIR"),
        "src/lexer/custom_lexer/custom_lexer.bytes",
    ]
    .iter()
    .collect::<PathBuf>();
    // ANCHOR: custom-lexer
    let bytes = std::fs::read(bytes_file).unwrap();
    let result = CustomLexer2Parser::new(MyCustomLexer2::new()).parse(&bytes);
    // ANCHOR_END: custom-lexer
    output_cmp!(
        "src/lexer/custom_lexer/custom_lexer_2.ast",
        format!("{:#?}", result)
    );
}
