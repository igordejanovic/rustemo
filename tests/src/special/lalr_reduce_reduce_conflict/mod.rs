use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/special/lalr_reduce_reduce_conflict");
rustemo_mod!(lang_actions, "/src/special/lalr_reduce_reduce_conflict");
use self::lang::LangParser;

#[test]
fn special_lalr_reduce_reduce_conflict() {
    let result = LangParser::new().parse("a c d");

    output_cmp!(
        "src/special/lalr_reduce_reduce_conflict/tree.ast",
        format!("{result:#?}")
    );
}
