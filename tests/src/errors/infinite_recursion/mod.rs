use rustemo_tools::output_cmp;

#[test]
fn infinite_recursion() {
    use crate::local_file;
    use indoc::indoc;
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "infinite_recursion.rustemo"));
    output_cmp!(
        "src/errors/infinite_recursion/infinite_recursion.err",
        result.unwrap_err().to_string()
    );
}
