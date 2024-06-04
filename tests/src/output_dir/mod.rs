use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::{local_file, output_cmp};

use self::output_dir::OutputDirParser;
use self::output_dir_act::OutputDirActParser;

// For the first test grammar both parser and actions are configured to be
// generated in the source tree.
#[rustfmt::skip]
#[allow(clippy::module_inception)]
mod output_dir;
#[rustfmt::skip]
#[allow(dead_code)]
mod output_dir_actions;

// For the second grammar only actions are generated in the source tree but the
// parser is generated in the Cargo output folder.
rustemo_mod!(output_dir_act, "/src/output_dir");
#[rustfmt::skip]
#[allow(dead_code)]
mod output_dir_act_actions;

#[test]
fn output_dir() {
    let result = OutputDirParser::new().parse("b b b 1");
    output_cmp!("src/output_dir/output_dir.ast", format!("{:#?}", result));

    // Both parser and actions are generated in the source tree
    assert!(local_file!(file!(), "output_dir.rs").exists());
    assert!(local_file!(file!(), "output_dir_actions.rs").exists());
}

#[test]
fn output_dir_act() {
    let result = OutputDirActParser::new().parse("b b b 1");
    output_cmp!(
        "src/output_dir/output_dir_act.ast",
        format!("{:#?}", result)
    );

    // Parser is not generated in the source tree, only actions
    assert!(!local_file!(file!(), "output_dir_act.rs").exists());
    assert!(local_file!(file!(), "output_dir_act_actions.rs").exists());
}
