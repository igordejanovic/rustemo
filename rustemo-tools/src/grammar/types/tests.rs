use crate::{
    grammar::{types::SymbolTypes, Grammar},
    output_cmp,
};

#[test]
fn symbols_type_deduction() {
    let grammar: Grammar = r#"
            A: myb=B c=C {MyKind}| B c=C | D {MyD} | Num | Recursive;
            B: C | EMPTY;
            C: b=B;
            D: a=A b=B  | mya=A B myb=B B D | EMPTY;
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

            ZeroOrMoreSugar1: Tc*;
            ZeroOrMoreSugar2: A*;
            ZeroOrMoreSugar3: A* | B C;
            ZeroOrMoreSugar4: A* B | B C;

            OneOrMoreSugar1: Tc+;
            OneOrMoreSugar2: A+;
            OneOrMoreSugar3: A+ | B C;
            OneOrMoreSugar4: A+ B | B C;

            OptionalSugar1: Tc?;
            OptionalSugar2: A?;
            OptionalSugar3: A? | B C;
            OptionalSugar4: A? B | B C;

            Recursive: RecursiveA | Tb;
            RecursiveA: Recursive Ta;

            terminals
            Num: /\d+/;
            Ta: 'a';
            Tb: 'b';
            Tc: 'c';
        "#
    .parse()
    .unwrap();

    let symbol_types = SymbolTypes::new(&grammar);
    output_cmp!(
        "src/grammar/types/symbols_type_deduction.expected",
        format!("{:#?}", symbol_types)
    );
}
