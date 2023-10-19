use rustemo::{rustemo_mod, GssHead, Parser, TreeBuilder};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/farshi_g7");
rustemo_mod!(
    #[allow(clippy::enum_variant_names)]
    lang_actions,
    "/src/glr/special/farshi_g7"
);
use self::lang::LangParser;

#[test]
fn glr_special_farshi_g7() {
    let forest = LangParser::new().parse("aaaaaaaaxbbcaacaa").unwrap();
    assert_eq!(forest.solutions(), 1);

    let tree = forest.get_first_tree();
    let mut builder = TreeBuilder::new();
    output_cmp!(
        "src/glr/special/farshi_g7/tree.ast",
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
