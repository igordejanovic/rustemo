use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

use self::partial::PartialParser;

rustemo_mod!(partial, "/src/partial");
rustemo_mod!(partial_actions, "/src/partial");

#[test]
fn partial_parse() {
    let result =
        PartialParser::parse("Numbers: 1 7 42 b b whatever .... bla bla");
    output_cmp!("src/partial/partial.ast", format!("{:#?}", result));
}
