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

            OptionalRef1: A | EMPTY;
            OptionalRef2: EMPTY | A;
            OptionalRef3: EMPTY | A 'c';

            Ref1: A;
            Ref2: A 'b';

            OptionalStruct1: A B | EMPTY;
            OptionalStruct2: EMPTY | mya=A B;
            Struct: A B;


            OptionalEnum1: A B | C | EMPTY;
            OptionalEnum2: A B | EMPTY | myc=C;
            Enum1: A B | myc=C;
            Enum2: A B | myc=C | 'a' 'b';


            MultiNonContent: 'a' | 'b' | 'c';
            MultiNonContentOptional: 'a' | 'b' | 'c' | EMPTY;

            @vec
            ZeroOrMore1: ZeroOrMore1 B | myb=B | EMPTY;
            @vec
            ZeroOrMore2: B z=ZeroOrMore2 | B | EMPTY;
            ZeroOrMore3: B ZeroOrMore3 | EMPTY | B ;

            OneOrMore1: OneOrMore1 B | B;
            @vec
            OneOrMore2: B OneOrMore2 | B;
            OneOrMore3: B | B OneOrMore3 ;

            ZeroOrMoreSuggar1: Tc*;
            ZeroOrMoreSuggar2: A*;
            ZeroOrMoreSuggar3: A* | B C;
            ZeroOrMoreSuggar4: A* B | B C;

            OneOrMoreSuggar1: Tc+;
            OneOrMoreSuggar2: A+;
            OneOrMoreSuggar3: A+ | B C;
            OneOrMoreSuggar4: A+ B | B C;

            OptionalSuggar1: Tc?;
            OptionalSuggar2: A?;
            OptionalSuggar3: A? | B C;
            OptionalSuggar4: A? B | B C;

            terminals
            Num: /\d+/;
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
