#[test]
fn infinite_recursion() {
    use crate::local_file;
    use indoc::indoc;
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "infinite_recursion.rustemo"));
    assert_eq!(
        indoc! {"
            Error at <str>:2:3:
            \tInfinite recursion on symbol 'B' in production '2:  B '."},
        result.unwrap_err().to_string(),
    );
}
