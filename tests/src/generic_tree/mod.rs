use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::generic_tree::GenericTreeParser;

// Only parser, no actions are generated for generic builder.
rustemo_mod!(generic_tree, "/src/generic_tree");

#[test]
fn generic_tree() {
    let result = GenericTreeParser::parse("a 42 a 3 b");
    output_cmp!(
        "src/generic_tree/generic_tree.ast",
        format!("{:#?}", result)
    );
}
