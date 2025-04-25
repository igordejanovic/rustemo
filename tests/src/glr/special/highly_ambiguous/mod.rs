use rustemo::{rustemo_mod, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/highly_ambiguous");
rustemo_mod!(lang_actions, "/src/glr/special/highly_ambiguous");
use self::lang::LangParser;

#[test]
fn glr_special_highly_ambiguous() {
    let forest = LangParser::new().parse("bbb").unwrap();
    assert_eq!(forest.solutions(), 3);

    let forest = LangParser::new().parse("bbbb").unwrap();
    assert_eq!(forest.solutions(), 10);

    (1..=forest.solutions()).for_each(|i| {
        let tree = forest.get_tree(i - 1);
        let mut builder = TreeBuilder::new();
        output_cmp!(
            &format!("src/glr/special/highly_ambiguous/tree_{i}.ast"),
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
