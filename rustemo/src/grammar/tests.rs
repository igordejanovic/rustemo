use crate::{
    grammar::{Associativity, Grammar},
    lang::rustemo_actions::Recognizer,
    tests::utils::type_of,
};
use rustemo_rt::index::ProdIndex;

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
    assert!(type_of(&grammar) == "rustemo::grammar::Grammar");
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
}

#[test]
fn nonterminals_productions() {
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
}
