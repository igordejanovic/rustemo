mod custom_builder_builder;

use rustemo::{rustemo_mod, Builder, Parser};
use rustemo_compiler::output_cmp;

use self::custom_builder::CustomBuilderParser;
use self::custom_builder_builder::MyCustomBuilder;

rustemo_mod!(custom_builder, "/src/builder/custom_builder");

#[test]
fn custom_builder() {
    // ANCHOR: custom-builder
    let result = CustomBuilderParser::new(MyCustomBuilder::new())
        .parse("2 + 4 * 5 + 20");
    // ANCHOR_END: custom-builder
    assert!(matches!(result, Ok(42)));
}
