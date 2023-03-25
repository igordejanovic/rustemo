use rustemo_compiler::{local_file, output_cmp};

#[test]
fn infinite_recursion() {
    let result = rustemo_compiler::with_settings()
        .process_grammar(local_file!(file!(), "infinite_recursion.rustemo"));
    output_cmp!(
        "src/errors/infinite_recursion/infinite_recursion.err",
        result.unwrap_err().to_string()
    );
}
