use rustemo::{rustemo_mod, Parser};

rustemo_mod!(lang, "/src/glr/special/knuth_lr1");
rustemo_mod!(lang_actions, "/src/glr/special/knuth_lr1");

use self::lang::LangParser;

#[test]
fn glr_special_knuth_lr1() {
    let forest = LangParser::new().parse("acccccccccd").unwrap();
    assert_eq!(forest.solutions(), 1);

    let forest = LangParser::new().parse("bcccccccccd").unwrap();
    assert_eq!(forest.solutions(), 1);
}
