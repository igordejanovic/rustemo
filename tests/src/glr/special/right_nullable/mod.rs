use rustemo::parser::Parser;
use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/right_nullable");
rustemo_mod!(lang_actions, "/src/glr/special/right_nullable");
use self::lang::LangParser;

#[test]
fn glr_special_right_nullable_g2() {
    let forest = LangParser::new().parse("aa").unwrap();
    assert_eq!(forest.solutions(), 1);
}
