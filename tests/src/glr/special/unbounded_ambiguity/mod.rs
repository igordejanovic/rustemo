use rustemo::{rustemo_mod, GssHead, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/unbounded_ambiguity");
rustemo_mod!(lang_actions, "/src/glr/special/unbounded_ambiguity");
use self::lang::LangParser;

#[test]
fn glr_special_unbounded_ambiguity() {
    let forest = LangParser::new().parse("xbbbbx").unwrap();
    assert_eq!(forest.solutions(), 5);

    (1..=forest.solutions()).for_each(|i| {
        let tree = forest.get_tree(i - 1);
        let mut builder = TreeBuilder::new();
        output_cmp!(
            &format!("src/glr/special/unbounded_ambiguity/tree_{}.ast", i),
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
    });
}
