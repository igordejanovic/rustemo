mod custom_builder_builder;

use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

use self::custom_builder::CustomBuilderParser;

rustemo_mod!(custom_builder, "/src/custom_builder");

#[test]
fn custom_builder() {
    let result = CustomBuilderParser::parse("2 + 4 * 5 + 20");
    assert!(matches!(result, Ok(42)));
}
