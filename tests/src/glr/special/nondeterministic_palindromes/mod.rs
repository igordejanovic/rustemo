use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/nondeterministic_palindromes");
rustemo_mod!(
    lang_actions,
    "/src/glr/special/nondeterministic_palindromes"
);
use self::lang::LangParser;

#[test]
fn glr_special_nondeterministic_palindromes() {
    let forest = LangParser::new().parse("01100100100110").unwrap();

    assert_eq!(forest.solutions(), 1);
    let result = forest.get_first_tree();

    output_cmp!(
        "src/glr/special/nondeterministic_palindromes/tree.ast",
        format!("{result:#?}")
    );
}
