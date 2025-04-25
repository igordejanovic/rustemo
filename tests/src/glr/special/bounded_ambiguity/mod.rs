use rustemo::{rustemo_mod, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/bounded_ambiguity");
rustemo_mod!(lang_actions, "/src/glr/special/bounded_ambiguity");
use self::lang::LangParser;

#[test]
fn glr_special_bounded_ambiguity() {
    let forest = LangParser::new().parse("xbbb").unwrap();
    assert_eq!(forest.solutions(), 2);

    (1..=forest.solutions()).for_each(|i| {
        let tree = forest.get_tree(i - 1);
        let mut builder = TreeBuilder::new();
        output_cmp!(
            &format!("src/glr/special/bounded_ambiguity/tree_{i}.ast"),
            format!(
                "{:#?}",
                tree.unwrap()
                    .build::<TreeBuilder<'_, str, lang::ProdKind, lang::TokenKind>, lang::State>(
                        &mut builder
                    )
            )
        );
    });
}
