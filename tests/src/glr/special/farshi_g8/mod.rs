use rustemo::{
    glr::gss::GssHead, lr::builder::TreeBuilder, parser::Parser, rustemo_mod,
};
use rustemo_compiler::output_cmp;

rustemo_mod!(lang, "/src/glr/special/farshi_g8");
rustemo_mod!(lang_actions, "/src/glr/special/farshi_g8");
use self::lang::LangParser;

#[test]
fn glr_special_farshi_g8() {
    let forest = LangParser::new().parse("xbbb").unwrap();
    assert_eq!(forest.solutions(), 8);

    (1..=forest.solutions()).for_each(|i| {
        let tree = forest.get_tree(i - 1);
        let mut builder = TreeBuilder::new();
        output_cmp!(
            &format!("src/glr/special/farshi_g8/tree_{}.ast", i),
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
