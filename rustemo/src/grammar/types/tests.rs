use crate::{grammar::{Grammar, types::SymbolTypes}, output_cmp};

#[test]
fn test_symbol_type_deduction() {
    let grammar = Grammar::from_string(
        r#"
            A: myb=B c=C {MyKind}| B c=C | D {MyD} | Num;
            B: C | EMPTY;
            C: b=B;
            D: a=A b=B  | mya=A B D | EMPTY;
            F: a=A F | D;

            Optional1: A | EMPTY;
            Optional2: EMPTY | A;

            Alias: A;

            MultiNonContent1: 'a' | 'b' | 'c';
            MultiNonContent2: 'a' | 'b' | 'c' | EMPTY;

            ZeroOrMore1: ZeroOrMore1 B | myb=B | EMPTY;
            ZeroOrMore2: B z=ZeroOrMore2 | B | EMPTY;
            ZeroOrMore3: B ZeroOrMore3 | EMPTY | B ;

            OneOrMore1: OneOrMore1 B | B;
            OneOrMore2: B OneOrMore2 | B;
            OneOrMore3: B | B OneOrMore3 ;

            terminals
            Num: "42";
            Ta: 'a';
            Tb: 'b';
            Tc: 'c';
        "#,
    )
    .unwrap();

    let symbol_types = SymbolTypes::new(&grammar);
    output_cmp!(
        "src/grammar/types/symbol_types_expected.txt",
        format!("{:#?}", symbol_types)
    );
}
