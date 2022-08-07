use crate::{grammar::{Grammar, types::SymbolTypes}, output_cmp};

#[test]
fn test_symbol_types() {
    let grammar = Grammar::from_string(
        r#"
            A: myb=B c=C {MyKind}| B c=C | D {MyD} | Num;
            B: C | EMPTY;
            C: b=B;
            D: a=A b=B  | mya=A B D | EMPTY;
            F: a=A F | D;
            terminals
            Num: "42";
        "#,
    )
    .unwrap();

    let symbol_types = SymbolTypes::new(&grammar);
    output_cmp!(
        "src/grammar/types/symbol_types_expected.txt",
        format!("{:#?}", symbol_types)
    );
}
