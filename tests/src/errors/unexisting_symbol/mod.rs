use rustemo_compiler::{local_file, output_cmp};

#[test]
fn unexisting() {
    let result = rustemo_compiler::process_grammar(local_file!(file!(), "unexisting.rustemo"));
    output_cmp!(
        "src/errors/unexisting_symbol/unexisting_symbol.err",
        result.unwrap_err().to_locfile_str()
    )
}
