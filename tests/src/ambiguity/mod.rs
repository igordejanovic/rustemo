use rustemo::rustemo_mod;
use rustemo_compiler::{local_file, output_cmp};

use self::reduce_empty_1::ReduceEmpty1Parser;
use self::reduce_empty_2::ReduceEmpty2Parser;

rustemo_mod!(reduce_empty_1, "/src/ambiguity");
rustemo_mod!(reduce_empty_1_actions, "/src/ambiguity");
rustemo_mod!(reduce_empty_2, "/src/ambiguity");
rustemo_mod!(reduce_empty_2_actions, "/src/ambiguity");

#[test]
fn reduce_empty_1() {
    let result = ReduceEmpty1Parser::new().parse("b b b");
    output_cmp!("src/ambiguity/reduce_empty_1.ast", format!("{:#?}", result));
}

#[test]
fn reduce_empty_2() {
    let result = ReduceEmpty2Parser::new().parse("1 42 2 b");
    output_cmp!("src/ambiguity/reduce_empty_2.ast", format!("{:#?}", result));
}

#[test]
fn prod_assoc_prio() {
    rustemo_compiler::process_grammar(local_file!(
        file!(),
        "prio_assoc_prod.rustemo"
    ))
    .unwrap();
}

#[test]
fn term_assoc_prod_prio() {
    rustemo_compiler::process_grammar(local_file!(
        file!(),
        "prio_assoc_term.rustemo"
    ))
    .unwrap();
}

#[test]
fn no_assoc_prod_conflicts() {
    let result = rustemo_compiler::process_grammar(local_file!(
        file!(),
        "no_prio_assoc_invalid.rustemo"
    ));
    output_cmp!(
        "src/ambiguity/no_prio_assoc_invalid.err",
        format!("{:#?}", result)
    );
}
