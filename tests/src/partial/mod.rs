use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

use self::partial::PartialParser;

rustemo_mod!(partial, "/src/partial");
rustemo_mod!(partial_actions, "/src/partial");

pub type Input = str;

#[test]
fn partial_parse() {
    let result =
        PartialParser::parse("Numbers: 1 7 42 b b whatever .... bla bla");
    output_cmp!("src/partial/partial.ast", format!("{:#?}", result));
}
