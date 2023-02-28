#[test]
fn terminal_not_defined() {
    use crate::local_file;
    use indoc::indoc;
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "terminal_not_defined.rustemo"));
    assert_eq!(
        indoc! {"
            Error at <str>:1:6:
            \tTerminal \"c\" used in production \"1:  B1  \"c\" \" is not defined in the 'terminals' section."},
        result.unwrap_err().to_string(),
    );
}

#[test]
fn terminal_not_defined_sugar() {
    use crate::local_file;
    use indoc::indoc;
    let result = rustemo_tools::with_settings().process_grammar(local_file!(
        file!(),
        "terminal_not_defined_sugar.rustemo"
    ));
    assert_eq!(
        indoc! {"
            Error at <str>:1:6:
            \tTerminal \"c\" is not defined in the terminals section."},
        result.unwrap_err().to_string(),
    );
}
