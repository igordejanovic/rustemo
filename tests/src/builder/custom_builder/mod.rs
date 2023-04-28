mod custom_builder_builder;

use rustemo::builder::Builder;
use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

use self::custom_builder::CustomBuilderParser;
use self::custom_builder_builder::MyCustomBuilder;

rustemo_mod!(custom_builder, "/src/builder/custom_builder");

#[test]
fn custom_builder() {
    let result = CustomBuilderParser::new(MyCustomBuilder::new())
        .parse("2 + 4 * 5 + 20");
    assert!(matches!(result, Ok(42)));
}
