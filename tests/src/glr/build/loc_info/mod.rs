use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::{local_file, output_cmp};
use serial_test::serial;

use self::json::JsonParser;

rustemo_mod!(json, "/src/glr/build/loc_info");
rustemo_mod!(json_actions, "/src/glr/build/loc_info");

#[test]
#[serial(loc_info)]
fn glr_loc_info() {
    let mut parser = JsonParser::new();
    let forest = parser
        // Using the same input file from LR test.
        .parse_file(local_file!(
            file!(),
            "../../../builder/loc_info/loc_info.json"
        ))
        .unwrap();

    output_cmp!(
        "src/glr/build/loc_info/loc_info_forest.ast",
        format!("{:#?}", forest)
    );

    let mut builder = self::json::DefaultBuilder::new();
    let result = forest.get_first_tree().unwrap().build(&mut builder);

    output_cmp!(
        // Using the same AST output from LR parser test as we expect the same result.
        "src/builder/loc_info/loc_info.ast",
        format!("{:#?}", result)
    );
}
