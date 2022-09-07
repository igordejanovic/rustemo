use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;
use std::fs;
use std::path::PathBuf;

use self::custom_lexer::CustomLexerParser;
//use self::custom_lexer_2::CustomLexerParser2;

mod custom_lexer_lexer;

rustemo_mod!(custom_lexer, "/src/custom_lexer");
mod custom_lexer_actions;
// rustemo_mod!(custom_lexer_2, "/src/custom_lexer");
// rustemo_mod!(custom_lexer_2_actions, "/src/custom_lexer");

#[test]
fn custom_lexer() {
    let bytes_file = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("custom_lexer.bytes");
    let bytes = std::fs::read(bytes_file).unwrap();
    let result = CustomLexerParser::parse(&*bytes);
    output_cmp!(
        "src/custom_lexer/custom_lexer.ast",
        format!("{:#?}", result)
    );
}
