use crate::{
    grammar::{Associativity, Grammar},
    lang::rustemo_actions::Recognizer,
    output_cmp,
    utils::type_of,
};
use rustemo::index::ProdIndex;

#[test]
fn grammar_from_string() {
    let grammar = Grammar::from_string(
        r#"
            S: A B;
        terminals
            A: "a";
            B: "b";
        "#,
    )
    .unwrap();
    assert!(type_of(&grammar) == "rustemo_tools::grammar::Grammar");
}

#[test]
fn create_terminals_1() {
    let grammar = Grammar::from_string(
        r#"
        S: "first_term" "second_term";
        terminals
        first_term: "first_term";
        second_term: "second_term";
        "#,
    )
    .unwrap();
    assert_eq!(
        grammar
            .terminals
            .iter()
            .map(|t| &t.name)
            .collect::<Vec<_>>(),
        &["STOP", "first_term", "second_term"]
    );
}

#[test]
fn create_terminals_2() {
    let grammar = Grammar::from_string(
        r#"
        S: "first_term" A "second_term";
        A: third_term;
        terminals
        first_term: "first_term";
        second_term: "second_term";
        third_term: ;
        "#,
    )
    .unwrap();
    assert_eq!(
        grammar
            .terminals
            .iter()
            .map(|t| &t.name)
            .collect::<Vec<_>>(),
        &["STOP", "first_term", "second_term", "third_term"]
    );
}

#[test]
fn create_terminals_multiple() {
    let grammar = Grammar::from_string(
        r#"
        S: "first_term" A "second_term" "first_term";
        A: third_term "third_term" "first_term" second_term;
        terminals
        first_term: "first_term";
        second_term: "second_term";
        third_term: "third_term";
        "#,
    )
    .unwrap();
    assert_eq!(
        grammar
            .terminals
            .iter()
            .map(|t| &t.name)
            .collect::<Vec<_>>(),
        &["STOP", "first_term", "second_term", "third_term"]
    );

    output_cmp!(
        "src/grammar/tests/create_terminals_multiple.expected",
        format!("{:#?}", grammar)
    );
}

#[test]
fn terminals_regex() {
    let grammar = Grammar::from_string(
        r#"
        S: "foo" rmatch_term A;
        A: "some" more_regex;
        terminals
        foo: "foo";
        some: "some";
        rmatch_term: /"[^"]+"/;
        more_regex: /\d{2,5}/;
        "#,
    )
    .unwrap();
    assert_eq!(
        grammar
            .terminals
            .iter()
            .map(|t| &t.name)
            .collect::<Vec<_>>(),
        &["STOP", "foo", "some", "rmatch_term", "more_regex"]
    );
    for (term_name, term_regex) in
        [("rmatch_term", r#""[^"]+""#), ("more_regex", r#"\d{2,5}"#)]
    {
        assert!(match grammar
            .symbol_to_term(grammar.term_by_name[term_name])
            .recognizer
            .as_ref()
            .unwrap()
        {
            Recognizer::StrConst(_) => false,
            Recognizer::RegexTerm(regex) => regex == term_regex,
        });
    }

    output_cmp!(
        "src/grammar/tests/terminals_regex.expected",
        format!("{:#?}", grammar)
    );
}

#[test]
fn nonterminal_productions() {
    let grammar = Grammar::from_string(
        r#"
        S: A "some_term" B | B;
        A: B;
        B: some_term;
        terminals
        some_term: "some_term";
        "#,
    )
    .unwrap();
    assert_eq!(grammar.nonterminals.len(), 5);
    assert_eq!(
        grammar
            .nonterminals
            .iter()
            .map(|nt| &nt.name)
            .collect::<Vec<_>>(),
        &["EMPTY", "AUG", "S", "A", "B"]
    );
    assert_eq!(
        grammar
            .nonterminals
            .iter()
            .map(|nt| nt.productions.len())
            .collect::<Vec<_>>(),
        &[0, 1, 2, 1, 1]
    );
    assert_eq!(
        grammar
            .nonterminals
            .iter()
            .flat_map(|nt| &nt.productions)
            .map(|index| {
                let ProdIndex(index) = index;
                *index
            })
            .collect::<Vec<_>>(),
        &[0, 1, 2, 3, 4]
    );
}

#[test]
fn productions_meta_data() {
    let grammar = Grammar::from_string(
        r#"
        S: A "some_term" B {5} | B {nops};
        A: B {nopse, bla: 5};
        B: some_term {right};
        terminals
        some_term: "some_term";
        "#,
    )
    .unwrap();
    assert_eq!(grammar.productions.len(), 5);

    assert_eq!(grammar.productions[ProdIndex(1)].prio, 5);
    assert_eq!(grammar.productions[ProdIndex(1)].meta.len(), 0);

    assert_eq!(grammar.productions[ProdIndex(2)].prio, 10);
    assert!(grammar.productions[ProdIndex(2)].nops);
    assert!(!grammar.productions[ProdIndex(2)].nopse);

    assert_eq!(grammar.productions[ProdIndex(3)].prio, 10);
    assert!(grammar.productions[ProdIndex(3)].nopse);
    assert_eq!(grammar.productions[ProdIndex(3)].meta.len(), 1);

    assert_eq!(
        grammar.productions[ProdIndex(4)].assoc,
        Associativity::Right
    );

    output_cmp!(
        "src/grammar/tests/productions_meta_data.expected",
        format!("{:#?}", grammar)
    );
}

#[test]
fn productions_meta_data_inheritance() {
    let grammar = Grammar::from_string(
        r#"
        S {15, nopse}: A "some_term" B {5} | B {nops};
        A {bla: 10}: B {nopse, bla: 5} | B {7};
        B {left}: some_term {right} | some_term;
        terminals
        some_term: "some_term";
        "#,
    )
    .unwrap();
    assert_eq!(grammar.productions.len(), 7);

    assert_eq!(grammar.productions[ProdIndex(1)].prio, 5);
    // Inherited
    assert!(grammar.productions[ProdIndex(1)].nopse);
    assert_eq!(grammar.productions[ProdIndex(1)].meta.len(), 0);

    // Inherited
    assert_eq!(grammar.productions[ProdIndex(2)].prio, 15);
    assert!(grammar.productions[ProdIndex(2)].nops);
    // Inherited
    assert!(grammar.productions[ProdIndex(2)].nopse);

    match grammar.productions[ProdIndex(3)].meta.get("bla").unwrap() {
        crate::lang::rustemo_actions::ConstVal::Int(i) => *i == 5u32,
        _ => unreachable!(),
    };
    assert_eq!(grammar.productions[ProdIndex(3)].meta.len(), 1);

    // Inherited
    assert_eq!(grammar.productions[ProdIndex(4)].prio, 7);
    match grammar.productions[ProdIndex(4)].meta.get("bla").unwrap() {
        crate::lang::rustemo_actions::ConstVal::Int(i) => *i == 10u32,
        _ => unreachable!(),
    };

    assert_eq!(
        grammar.productions[ProdIndex(5)].assoc,
        Associativity::Right
    );

    // Inherited
    assert_eq!(grammar.productions[ProdIndex(6)].assoc, Associativity::Left);
}

#[test]
fn regex_sugar_zero_or_more() {
    let grammar = Grammar::from_string(
        r#"
        S: A* B | C* | "some"*;
        A: Some A*;
        B: Some | EMPTY;
        C: A* Some;
        terminals
        Some: "some";
        "#,
    )
    .unwrap();
    output_cmp!(
        "src/grammar/tests/regex_sugar_zero_or_more.expected",
        format!("{:#?}", grammar)
    );
}

#[test]
fn regex_sugar_one_or_more() {
    let grammar = Grammar::from_string(
        r#"
        S: A+ B | C+ | "some"+;
        A: Some A+;
        B: Some | EMPTY;
        C: A+ Some;
        terminals
        Some: "some";
        "#,
    )
    .unwrap();
    output_cmp!(
        "src/grammar/tests/regex_sugar_one_or_more.expected",
        format!("{:#?}", grammar)
    );
}

#[test]
fn regex_sugar_optional() {
    let grammar = Grammar::from_string(
        r#"
        S: A? B | C? | "some"?;
        A: Some A?;
        B: Some | EMPTY;
        C: A? Some;
        terminals
        Some: "some";
        "#,
    )
    .unwrap();
    output_cmp!(
        "src/grammar/tests/regex_sugar_optional.expected",
        format!("{:#?}", grammar)
    );
}
