use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

use self::unicode::UnicodeParser;

rustemo_mod!(unicode, "/src/unicode");
rustemo_mod!(unicode_actions, "/src/unicode");

#[test]
fn partial_parse() {
    let result = UnicodeParser::new()
        .parse("Тестирање: čokančićem ћу те, чоканчићем ћеш ме.");
    output_cmp!("src/unicode/unicode.ast", format!("{:#?}", result));
}
