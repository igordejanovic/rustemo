use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::{local_file, output_cmp};

use self::fancy_regex::FancyRegexParser;

rustemo_mod!(fancy_regex, "/src/fancy_regex");
rustemo_mod!(fancy_regex_actions, "/src/fancy_regex");

#[test]
fn fancy_regex() {
    let result = FancyRegexParser::new().parse("foo foo 42 27 13");
    output_cmp!("src/fancy_regex/fancy_regex.ast", format!("{:#?}", result));
}
