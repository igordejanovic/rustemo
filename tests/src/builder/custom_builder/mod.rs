mod custom_builder_builder;

use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

use self::custom_builder::CustomBuilderParser;

rustemo_mod!(custom_builder, "/src/builder/custom_builder");

#[test]
fn custom_builder() {
    let result = CustomBuilderParser::parse("2 + 4 * 5 + 20");
    assert!(matches!(result, Ok(42)));
}
