use rustemo::parser::Parser;
use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/cyclic_1");
rustemo_mod!(lang_actions, "/src/glr/special/cyclic_1");
use self::lang::LangParser;

#[test]
fn glr_special_cyclic_1() {
    let forest = LangParser::new().parse("x").unwrap();

    // This will cause stack overflow as the forest is circular
    //
    // TODO: Detection of circular forest should be implemented and a panic with
    // the cause should be done.
    //assert_eq!(forest.solutions(), 1);
}
