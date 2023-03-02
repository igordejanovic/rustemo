use rustemo_tools::output_cmp;

#[test]
fn unexisting() {
    use crate::local_file;
    let result = rustemo_tools::with_settings()
        .process_grammar(local_file!(file!(), "unexisting.rustemo"));
    output_cmp!(
        "src/errors/unexisting_symbol/unexisting_symbol.err",
        result.unwrap_err().to_string()
    )
}
