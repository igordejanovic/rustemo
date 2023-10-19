use rustemo::{rustemo_mod, GssHead, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/reduce_enough_empty");
rustemo_mod!(lang_actions, "/src/glr/special/reduce_enough_empty");
use self::lang::LangParser;

#[test]
fn glr_special_reduce_enough_empty() {
    let forest = LangParser::new().parse("xbbb").unwrap();
    assert_eq!(forest.solutions(), 1);

    let tree = forest.get_first_tree();
    let mut builder = TreeBuilder::new();
    output_cmp!(
        "src/glr/special/reduce_enough_empty/tree.ast",
        format!(
            "{:#?}",
            tree.unwrap().build::<TreeBuilder<
                '_,
                str,
                lang::ProdKind,
                lang::TokenKind,
            >, GssHead<'_, str, lang::State, lang::TokenKind>, lang::State>(
                &mut builder
            )
        )
    );
}
