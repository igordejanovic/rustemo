use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/cyclic_2");
rustemo_mod!(lang_actions, "/src/glr/special/cyclic_2");
use self::lang::LangParser;

#[test]
#[ignore]
fn glr_special_cyclic_2() {
    let forest = LangParser::new().parse("x").unwrap();

    // This will cause stack overflow as the forest is circular
    //
    // TODO: Detection of circular forest should be implemented and a panic with
    // the cause should be done.
    assert_eq!(forest.solutions(), 1);
}
