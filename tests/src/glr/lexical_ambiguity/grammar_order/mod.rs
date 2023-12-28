use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(grammar_order, "/src/glr/lexical_ambiguity/grammar_order");
rustemo_mod!(
    grammar_order_actions,
    "/src/glr/lexical_ambiguity/grammar_order"
);

use self::grammar_order::GrammarOrderParser;

#[test]
fn glr_lexical_ambiguity_grammar_order() {
    let forest = GrammarOrderParser::new().parse("s a 42.42").unwrap();
    assert_eq!(forest.solutions(), 1);

    let mut trees = String::new();
    for tree in &forest {
        trees.push_str(&format!("{tree:#?}\n\n"));
    }
    output_cmp!(
        "src/glr/lexical_ambiguity/grammar_order/grammar_order.ast",
        format!("{trees}")
    );
}
