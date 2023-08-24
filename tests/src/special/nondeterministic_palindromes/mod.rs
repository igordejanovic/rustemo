use rustemo::parser::Parser;
use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/special/nondeterministic_palindromes");
rustemo_mod!(lang_actions, "/src/special/nondeterministic_palindromes");
use self::lang::LangParser;

#[test]
fn special_nondeterministic_palindromes() {
    let result = LangParser::new().parse("01100100100110");

    output_cmp!(
        "src/special/nondeterministic_palindromes/message.err",
        format!("{result:#?}")
    );
}
