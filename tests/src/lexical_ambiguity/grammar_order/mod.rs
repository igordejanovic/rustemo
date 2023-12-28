use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(grammar_order, "/src/lexical_ambiguity/grammar_order");
rustemo_mod!(
    grammar_order_actions,
    "/src/lexical_ambiguity/grammar_order"
);

use self::grammar_order::GrammarOrderParser;

#[test]
fn lr_lexical_ambiguity_grammar_order() {
    let result = GrammarOrderParser::new().parse("s a 42.42").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/grammar_order/grammar_order.ast",
        format!("{result:?}")
    );
}
