use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::{local_file, output_cmp};
use serial_test::serial;

use self::json::JsonParser;

rustemo_mod!(json, "/src/builder/loc_info");
rustemo_mod!(json_actions, "/src/builder/loc_info");

#[test]
#[serial(loc_info)]
fn loc_info() {
    let mut parser = JsonParser::new();
    let result = parser
        .parse_file(local_file!(file!(), "loc_info.json"))
        .unwrap();
    output_cmp!(
        "src/builder/loc_info/loc_info.ast",
        format!("{:#?}", result)
    );
}
