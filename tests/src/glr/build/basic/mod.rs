use rustemo::{rustemo_mod, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/build/basic");
rustemo_mod!(calc_actions, "/src/glr/build/basic");

use self::calc::CalcParser;

// ANCHOR: build
#[test]
fn glr_tree_build_default() {
    let forest = CalcParser::new().parse("1 + 4 * 9").unwrap();
    assert_eq!(forest.solutions(), 2);

    let mut builder = calc::DefaultBuilder::new();
    output_cmp!(
        "src/glr/build/basic/tree_build_default_1.ast",
        format!(
            "{:#?}",
            forest.get_first_tree().unwrap().build(&mut builder)
        )
    );
    output_cmp!(
        "src/glr/build/basic/tree_build_default_2.ast",
        format!("{:#?}", forest.get_tree(1).unwrap().build(&mut builder))
    );
}
// ANCHOR_END: build

#[test]
fn glr_tree_build_generic() {
    let forest = CalcParser::new().parse("1 + 4 * 9").unwrap();
    assert_eq!(forest.solutions(), 2);

    let mut builder = TreeBuilder::new();
    output_cmp!(
        "src/glr/build/basic/tree_build_generic_1.ast",
        format!(
            "{:#?}",
            forest
                .get_first_tree()
                .unwrap()
                .build::<TreeBuilder<'_, str, calc::ProdKind, calc::TokenKind>, calc::State>(
                    &mut builder
                )
        )
    );
    output_cmp!(
        "src/glr/build/basic/tree_build_generic_2.ast",
        format!(
            "{:#?}",
            forest
                .get_tree(1)
                .unwrap()
                .build::<TreeBuilder<'_, str, calc::ProdKind, calc::TokenKind>, calc::State>(
                    &mut builder
                )
        )
    );
}
