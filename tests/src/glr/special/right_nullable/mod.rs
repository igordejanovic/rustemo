use rustemo::{rustemo_mod, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/right_nullable");
rustemo_mod!(lang_actions, "/src/glr/special/right_nullable");
use self::lang::LangParser;

#[test]
fn glr_special_right_nullable_g2() {
    let forest = LangParser::new().parse("aa").unwrap();
    assert_eq!(forest.solutions(), 2);
    (1..=forest.solutions()).for_each(|i| {
        let tree = forest.get_tree(i - 1);
        let mut builder = TreeBuilder::new();
        output_cmp!(
            &format!("src/glr/special/right_nullable/tree_{}.ast", i),
            format!(
                "{:#?}",
                tree.unwrap()
                    .build::<TreeBuilder<'_, str, lang::ProdKind, lang::TokenKind>, lang::State>(
                        &mut builder
                    )
            )
        );
    })
}
