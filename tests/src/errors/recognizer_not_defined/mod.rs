use rustemo_compiler::{local_file, output_cmp, LexerType::Custom};

#[test]
fn recognizer_not_defined_for_default_lexer() {
    let result = rustemo_compiler::process_grammar(local_file!(
        file!(),
        "recognizer_not_defined.rustemo"
    ));
    output_cmp!(
        "src/errors/recognizer_not_defined/recognizer_not_defined.err",
        result.unwrap_err().to_string()
    );
}

/// In custom lexer is used recognizers don't need to be defined.
#[test]
fn recognizer_not_defined_for_custom_lexer() {
    let result = rustemo_compiler::Settings::new()
        .lexer_type(Custom)
        .process_grammar(local_file!(
            file!(),
            "recognizer_not_defined.rustemo"
        ));
    result.unwrap();
}
