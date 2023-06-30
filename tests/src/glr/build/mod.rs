use std::borrow::BorrowMut;

use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/build");
rustemo_mod!(calc_actions, "/src/glr/build");

use self::calc::CalcParser;
use rustemo::parser::Parser;

#[test]
fn glr_tree_build() {
    let forest = CalcParser::new().parse("1 + 4 * 9").unwrap();
    assert_eq!(forest.solutions(), 2);

    let mut builder = calc::DefaultBuilder::new();
    output_cmp!(
        "src/glr/tree_build_1.ast",
        format!(
            "{:#?}",
            forest.get_first_tree().unwrap().build(&mut builder)
        )
    );
    output_cmp!(
        "src/glr/tree_build_2.ast",
        format!("{:#?}", forest.get_tree(1).unwrap().build(&mut builder))
    );
}
