#[test]
fn unexisting() {
    use crate::local_file;
    use indoc::indoc;
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "unexisting.rustemo"));
    assert_eq!(
        indoc! {"
            Error at <str>:2:3:
            \tUnexisting symbol 'C' in production '2:  C  Tc '."},
        result.unwrap_err().to_string(),
    );
}
