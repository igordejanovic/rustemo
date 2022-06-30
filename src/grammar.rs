use std::{
    collections::BTreeMap,
    fmt::Display,
    hash::{Hash, Hasher},
};

use rustemort::{index::{
    NonTermIndex, NonTermVec, ProdIndex, ProdVec, SymbolIndex, SymbolVec,
    TermIndex, TermVec,
}, log};

use super::rustemo_actions::{
    GrammarRule, GrammarSymbol, Imports, PGFile, ProductionMetaDatas,
    Recognizer, TerminalMetaDatas,
};

#[derive(Debug)]
pub struct Grammar {
    pub imports: Option<Imports>,
    pub productions: Option<ProdVec<Production>>,
    pub terminals: Option<TermVec<Terminal>>,
    pub nonterminals: Option<NonTermVec<NonTerminal>>,
    pub nonterm_by_name: BTreeMap<String, SymbolIndex>,
    pub term_by_name: BTreeMap<String, SymbolIndex>,
    // Index of EMPTY symbol
    pub empty_index: SymbolIndex,
    // Index of STOP symbol
    pub stop_index: SymbolIndex,
    // Index of grammar augmented symbol
    pub augmented_index: SymbolIndex,
}

macro_rules! grammar_elem {
    ($name:ident) => {
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.idx == other.idx
            }
        }
        impl Eq for $name {}
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.idx.hash(state);
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.idx.partial_cmp(&other.idx)
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.idx.cmp(&other.idx)
            }
        }
    };
}

#[derive(Debug)]
pub struct Terminal {
    pub idx: TermIndex,
    pub name: String,
    pub action: Option<String>,
    pub recognizer: Option<Recognizer>,
    pub meta: TerminalMetaDatas,
}
grammar_elem!(Terminal);

#[derive(Debug)]
pub struct NonTerminal {
    pub idx: NonTermIndex,
    pub name: String,
    pub productions: Vec<ProdIndex>,
}
grammar_elem!(NonTerminal);

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nGRAMMAR [")?;
        writeln!(f, "\nTerminals:")?;
        for terminal in self.terminals() {
            writeln!(f, "{}. {}", terminal.idx, terminal.name)?;
        }
        writeln!(f, "\nNonTerminals:")?;
        for nonterminal in self.nonterminals() {
            writeln!(
                f,
                "{} ({}). {}",
                nonterminal.idx,
                self.nonterm_to_symbol(nonterminal.idx),
                nonterminal.name
            )?;
        }
        writeln!(f, "\nProductions:")?;
        for production in self.productions() {
            write!(
                f,
                "{}. {}: ",
                production.idx,
                self.nonterminals()[production.nonterminal].name
            )?;
            for assignment in &production.rhs {
                write!(f, "{} ", self.symbol_name(res_symbol(assignment)))?;
            }
            writeln!(f, "")?;
        }

        writeln!(f, "\n] GRAMMAR")
    }
}

#[derive(Debug, PartialEq)]
pub enum Associativity {
    None,
    Left,
    Right,
}

impl Default for Associativity {
    fn default() -> Self {
        Associativity::None
    }
}

pub type Priority = u32;
pub const DEFAULT_PRIORITY: u32 = 10;

#[derive(Debug)]
pub struct Production {
    pub idx: ProdIndex,
    pub nonterminal: NonTermIndex,
    pub rhs: Vec<Assignment>,
    pub assoc: Associativity,
    pub prio: Priority,
    pub dynamic: bool,
    pub nops: bool,
    pub nopse: bool,
    pub meta: ProductionMetaDatas,
}
grammar_elem!(Production);

impl Default for Production {
    fn default() -> Self {
        Self {
            idx: Default::default(),
            nonterminal: Default::default(),
            rhs: Default::default(),
            assoc: Default::default(),
            prio: DEFAULT_PRIORITY,
            dynamic: Default::default(),
            nops: Default::default(),
            nopse: Default::default(),
            meta: Default::default(),
        }
    }
}

impl Production {
    pub fn rhs_symbols(&self) -> Vec<SymbolIndex> {
        self.rhs.iter().map(|a| res_symbol(a)).collect()
    }
    pub fn rhs_symbol(&self, pos: usize) -> SymbolIndex {
        res_symbol(&self.rhs[pos])
    }
}

#[derive(Debug)]
pub enum ResolvingSymbolIndex {
    Resolved(SymbolIndex),
    Resolving(GrammarSymbol),
}

#[derive(Debug)]
pub struct Assignment {
    pub name: Option<String>,
    pub symbol: ResolvingSymbolIndex,
}

/// Called for Assignment to extract resolved SymbolIndex.
#[inline]
pub(in crate) fn res_symbol(assign: &Assignment) -> SymbolIndex {
    match assign.symbol {
        ResolvingSymbolIndex::Resolved(index) => index,
        ResolvingSymbolIndex::Resolving(_) => {
            // This shouldn't happen
            panic!("reference {:?} not resolved", assign.symbol);
        }
    }
}

impl Grammar {
    pub fn from_pgfile(pgfile: PGFile) -> Self {
        let mut terminals: BTreeMap<String, Terminal> = BTreeMap::new();
        let mut nonterminals: BTreeMap<String, NonTerminal> = BTreeMap::new();
        let mut productions: ProdVec<Production> = ProdVec::new();

        // Create implicit STOP terminal used to signify the end of the input.
        terminals.insert(
            "STOP".to_string(),
            Terminal {
                idx: TermIndex(0),
                name: "STOP".to_string(),
                action: None,
                recognizer: None,
                meta: TerminalMetaDatas::new(),
            },
        );

        // Extract productions and nonterminals from grammar rules.
        if let Some(rules) = pgfile.rules {
            Grammar::extract_productions_and_symbols(
                rules,
                &mut nonterminals,
                &mut productions,
            );
        }

        // TODO: Desugaring. Related to the previous. Desugar repetitions and
        // groups.

        // Collect grammar terminals
        if let Some(grammar_terminals) = pgfile.terminals {
            Grammar::collect_terminals(grammar_terminals, &mut terminals);
        }

        // Create implicit terminals from string constants.
        Grammar::create_terminals_from_productions(
            &productions,
            &mut terminals,
        );

        // Resolve references in productions.
        Grammar::resolve_references(
            &mut productions,
            &terminals,
            &nonterminals,
        );

        let term_len = terminals.len();
        let grammar = Grammar {
            imports: pgfile.imports,
            productions: Some(productions),
            empty_index: terminals.len().into(), // Right after the last terminal
            augmented_index: (terminals.len() + 1).into(), // skip EMPTY
            stop_index: 0.into(),
            term_by_name: terminals
                .values()
                .map(|t| (t.name.to_string(), t.idx.to_symbol_index()))
                .collect(),
            terminals: if terminals.is_empty() {
                None
            } else {
                let mut terms: TermVec<_> = terminals.into_values().collect();
                terms.sort();
                Some(terms)
            },
            nonterm_by_name: nonterminals
                .values()
                .map(|nt| {
                    (nt.name.to_string(), nt.idx.to_symbol_index(term_len))
                })
                .collect(),
            nonterminals: if nonterminals.is_empty() {
                None
            } else {
                let mut nonterms: NonTermVec<_> =
                    nonterminals.into_values().collect();
                nonterms.sort();
                Some(nonterms)
            },
        };
        // TODO: Dump only if tracing is used
        log!("{grammar}");
        grammar
    }

    fn extract_productions_and_symbols(
        rules: Vec<GrammarRule>,
        nonterminals: &mut BTreeMap<String, NonTerminal>,
        productions: &mut ProdVec<Production>,
    ) {
        let mut last_nonterm_idx = NonTermIndex(1); // Account for EMPTY and S'
        let mut next_prod_idx = ProdIndex(1); // Account for S' -> S production
        let mut nonterminal;

        // EMPTY non-terminal is implicit
        nonterminals.insert(
            "EMPTY".to_string(),
            NonTerminal {
                idx: NonTermIndex(0),
                name: "EMPTY".to_string(),
                productions: vec![],
            },
        );

        // Augmented non-terminal and production. by default first rule is
        // starting rule.
        nonterminals.insert(
            "S'".to_string(),
            NonTerminal {
                idx: NonTermIndex(1),
                name: "S'".to_string(),
                productions: vec![ProdIndex(0)],
            },
        );

        // Add augmented S' -> S production
        productions.push(Production {
            idx: ProdIndex(0),
            nonterminal: NonTermIndex(1),
            rhs: vec![Assignment {
                name: None,
                symbol: ResolvingSymbolIndex::Resolving(GrammarSymbol::Name(
                    rules[0].name.to_string(),
                )),
            }],
            ..Production::default()
        });

        for rule in rules {
            // Crate or find non-terminal for the current rule
            nonterminal = nonterminals
                .entry(rule.name.to_string())
                .or_insert_with(|| {
                    last_nonterm_idx.0 += 1;
                    NonTerminal {
                        idx: last_nonterm_idx,
                        name: rule.name.to_string(),
                        productions: vec![],
                    }
                });

            // Gather productions, create indexes. Transform RHS to mark
            // resolving references.
            for production in rule.rhs {
                let mut new_production = Production {
                    idx: next_prod_idx,
                    nonterminal: nonterminal.idx,
                    rhs: production
                        .assignments
                        .into_iter()
                        .map(|assignment| {
                            use super::rustemo_actions::Assignment::*;
                            match assignment {
                                PlainAssignment(assign)
                                | BoolAssignment(assign) => Assignment {
                                    name: Some(assign.name),
                                    symbol: ResolvingSymbolIndex::Resolving(
                                        assign.gsymref.gsymbol.unwrap(),
                                    ),
                                },
                                GSymbolReference(reference) => Assignment {
                                    name: None,
                                    symbol: ResolvingSymbolIndex::Resolving(
                                        reference.gsymbol.unwrap(),
                                    ),
                                },
                            }
                        })
                        .collect(),
                    meta: production.meta,
                    ..Production::default()
                };

                // Map meta-data to production fields for easier access
                if let Some(meta) = new_production.meta.remove("priority") {
                    new_production.prio = match meta {
                        crate::rustemo_actions::Const::Int(p) => p,
                        _ => panic!("Invalid Const!"),
                    }
                }

                if let Some(_) = new_production.meta.remove("left") {
                    new_production.assoc = Associativity::Left;
                }
                if let Some(_) = new_production.meta.remove("right") {
                    new_production.assoc = Associativity::Right;
                }
                if let Some(_) = new_production.meta.remove("nops") {
                    new_production.nops = true;
                }
                if let Some(_) = new_production.meta.remove("nopse") {
                    new_production.nopse = true;
                }

                productions.push(new_production);
                nonterminal.productions.push(next_prod_idx);
                next_prod_idx.0 += 1;
            }
        }
    }

    fn collect_terminals(
        grammar_terminals: Vec<super::rustemo_actions::Terminal>,
        terminals: &mut BTreeMap<String, Terminal>,
    ) {
        let mut next_term_idx = TermIndex(1); // Account for STOP terminal
        for terminal in grammar_terminals {
            terminals.insert(
                terminal.name.to_string(),
                Terminal {
                    idx: next_term_idx,
                    name: terminal.name,
                    action: terminal.action,
                    recognizer: terminal.recognizer,
                    meta: terminal.meta,
                },
            );
            next_term_idx.0 += 1;
        }
    }

    fn create_terminals_from_productions(
        productions: &ProdVec<Production>,
        terminals: &mut BTreeMap<String, Terminal>,
    ) {
        let mut next_term_idx = TermIndex(terminals.len());
        for production in productions {
            for assign in &production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol
                {
                    if let GrammarSymbol::StrConst(name) = symbol {
                        if !terminals.contains_key(name) {
                            terminals.insert(
                                name.to_string(),
                                Terminal {
                                    idx: next_term_idx,
                                    name: name.to_string(),
                                    action: None,
                                    recognizer: Some(Recognizer::StrConst(
                                        name.to_string(),
                                    )),
                                    meta: TerminalMetaDatas::new(),
                                },
                            );
                            next_term_idx.0 += 1;
                        }
                    }
                }
            }
        }
    }

    fn resolve_references(
        productions: &mut ProdVec<Production>,
        terminals: &BTreeMap<String, Terminal>,
        nonterminals: &BTreeMap<String, NonTerminal>,
    ) {
        // Resolve references.
        for production in productions {
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol
                {
                    match symbol {
                        GrammarSymbol::Name(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                if let Some(terminal) = terminals.get(name) {
                                    terminal.idx.to_symbol_index()
                                } else {
                                    nonterminals
                                        .get(name)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "unexisting symbol {:?}.",
                                                name
                                            )
                                        })
                                        .idx
                                        .to_symbol_index(terminals.len())
                                },
                            )
                        }
                        GrammarSymbol::StrConst(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                terminals
                                    .get(name)
                                    .unwrap_or_else(|| {
                                        panic!(
                                            "terminal {:?} not created!.",
                                            name
                                        )
                                    })
                                    .idx
                                    .to_symbol_index(),
                            )
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn new_termvec<T: Clone>(&self, default: T) -> TermVec<T> {
        TermVec(vec![default; self.term_len()])
    }

    pub(crate) fn new_nontermvec<T: Clone>(&self, default: T) -> NonTermVec<T> {
        NonTermVec(vec![default; self.nonterm_len()])
    }

    pub(crate) fn symbol_index(&self, name: &str) -> SymbolIndex {
        *self.term_by_name.get(name).unwrap_or_else(|| {
            self.nonterm_by_name.get(name).unwrap_or_else(|| {
                panic!("No Symbol by name {:?}", name);
            })
        })
    }

    pub(crate) fn symbol_name(&self, index: SymbolIndex) -> String {
        if index.0 < self.term_len() {
            self.terminals.as_ref().unwrap()[self.symbol_to_term(index)]
                .name
                .clone()
        } else {
            self.nonterminals.as_ref().unwrap()[self.symbol_to_nonterm(index)]
                .name
                .clone()
        }
    }

    pub(crate) fn symbol_indexes(
        &self,
        names: &[&str],
    ) -> SymbolVec<SymbolIndex> {
        let mut indexes = SymbolVec::new();
        for name in names {
            indexes.push(self.symbol_index(name))
        }
        indexes
    }

    pub(crate) fn symbol_names<'a, T>(&self, indexes: T) -> Vec<String>
    where
        T: IntoIterator<Item = &'a SymbolIndex>,
    {
        indexes
            .into_iter()
            .copied()
            .map(|i| self.symbol_name(i))
            .collect()
    }

    #[inline]
    pub(crate) fn term_to_symbol(&self, index: TermIndex) -> SymbolIndex {
        SymbolIndex(index.0)
    }

    /// Convert symbol index to terminal index.
    #[inline]
    pub(crate) fn symbol_to_term(&self, index: SymbolIndex) -> TermIndex {
        TermIndex(index.0)
    }

    #[inline]
    pub(crate) fn nonterm_to_symbol(&self, index: NonTermIndex) -> SymbolIndex {
        SymbolIndex(index.0 + self.term_len())
    }

    /// Convert symbol index to non-terminal index. Panics if symbol index is a
    /// terminal index.
    #[inline]
    pub(crate) fn symbol_to_nonterm(&self, index: SymbolIndex) -> NonTermIndex {
        NonTermIndex(index.0.checked_sub(self.term_len()).unwrap())
    }

    #[inline]
    pub(crate) fn is_nonterm(&self, index: SymbolIndex) -> bool {
        index.0 >= self.term_len()
    }

    #[inline]
    pub(crate) fn is_term(&self, index: SymbolIndex) -> bool {
        index.0 < self.term_len()
    }

    /// Number of terminals in the grammar.
    #[inline]
    pub(crate) fn term_len(&self) -> usize {
        self.terminals.as_ref().map_or(0, |t| t.len())
    }

    /// Number of non-terminals in the grammar including EMPTY and S'
    #[inline]
    pub(crate) fn nonterm_len(&self) -> usize {
        self.nonterminals.as_ref().map_or(0, |nt| nt.len())
    }

    #[inline]
    pub(crate) fn terminals(&self) -> &TermVec<Terminal> {
        self.terminals.as_ref().unwrap()
    }

    #[inline]
    pub(in crate) fn nonterminals(&self) -> &NonTermVec<NonTerminal> {
        self.nonterminals.as_ref().unwrap()
    }

    #[inline]
    pub(crate) fn productions(&self) -> &ProdVec<Production> {
        self.productions.as_ref().unwrap()
    }

    #[inline]
    pub(crate) fn production_len(&self, prod: ProdIndex) -> usize {
        self.productions()[prod].rhs.len()
    }

    #[inline]
    pub(crate) fn production_rhs_symbols(
        &self,
        prod: ProdIndex,
    ) -> Vec<SymbolIndex> {
        self.productions()[prod]
            .rhs
            .iter()
            .map(|assgn| res_symbol(assgn))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{grammar::Associativity, rustemo::RustemoParser};
    use rustemort::index::ProdIndex;

    #[test]
    fn create_terminals_1() {
        let grammar = RustemoParser::default().parse(
            r#"
            S: "first_term" "second_term";
            "#
            .into(),
        );
        assert_eq!(
            grammar
                .terminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>(),
            &["STOP", "first_term", "second_term"]
        );
    }

    #[test]
    fn create_terminals_2() {
        let grammar = RustemoParser::default().parse(
            r#"
            S: "first_term" A "second_term";
            A: third_term;
            terminals
            third_term: ;
            "#
            .into(),
        );
        assert_eq!(
            grammar
                .terminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>(),
            // `third_term` is collected first and gets lower TermIndex (1)
            &["STOP", "third_term", "first_term", "second_term"]
        );
    }

    #[test]
    fn create_terminals_multiple() {
        let grammar = RustemoParser::default().parse(
            r#"
            S: "first_term" A "second_term" "first_term";
            A: third_term "third_term" "first_term" second_term;
            terminals
            third_term: ;
            "#
            .into(),
        );
        assert_eq!(
            grammar
                .terminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>(),
            // `third_term` is collected first and gets lower TermIndex (1)
            &["STOP", "third_term", "first_term", "second_term"]
        );
    }

    #[test]
    fn terminals_regex() {
        let grammar = RustemoParser::default().parse(
            r#"
            S: "foo" rmatch_term A;
            A: "some" "more_regex";
            terminals
            rmatch_term: /"[^"]+"/;
            more_regex: /\d{2,5}/;
            "#
            .into(),
        );
        assert_eq!(
            grammar
                .terminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>(),
            &["STOP", "rmatch_term", "more_regex", "foo", "some"]
        );
        for (term_name, term_regex) in
            [("rmatch_term", r#""[^"]+""#), ("more_regex", r#"\d{2,5}"#)]
        {
            assert!(match grammar.terminals.as_ref().unwrap()
                [grammar.symbol_to_term(grammar.term_by_name[term_name])]
            .recognizer
            .as_ref()
            .unwrap()
            {
                crate::rustemo_actions::Recognizer::StrConst(_) => false,
                crate::rustemo_actions::Recognizer::RegExTerm(regex) =>
                    regex == term_regex,
            });
        }
    }

    #[test]
    fn nonterminals_productions() {
        let grammar = RustemoParser::default().parse(
            r#"
            S: A "some_term" B | B;
            A: B;
            B: some_term;
            "#
            .into(),
        );
        assert_eq!(grammar.nonterminals().len(), 5);
        assert_eq!(
            grammar
                .nonterminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|nt| &nt.name)
                .collect::<Vec<_>>(),
            &["EMPTY", "S'", "S", "A", "B"]
        );
        assert_eq!(
            grammar
                .nonterminals
                .as_ref()
                .unwrap()
                .iter()
                .map(|nt| nt.productions.len())
                .collect::<Vec<_>>(),
            &[0, 1, 2, 1, 1]
        );
        assert_eq!(
            grammar
                .nonterminals
                .as_ref()
                .unwrap()
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
        let grammar = RustemoParser::default().parse(
            r#"
            S: A "some_term" B {5} | B {nops};
            A: B {nopse, bla: 5};
            B: some_term {right};
            "#
            .into(),
        );
        assert_eq!(grammar.productions().len(), 5);

        assert_eq!(grammar.productions()[1.into()].prio, 5);
        assert_eq!(grammar.productions()[1.into()].meta.len(), 0);

        assert_eq!(grammar.productions()[2.into()].prio, 10);
        assert!(grammar.productions()[2.into()].nops);
        assert!(!grammar.productions()[2.into()].nopse);

        assert_eq!(grammar.productions()[3.into()].prio, 10);
        assert!(grammar.productions()[3.into()].nopse);
        assert_eq!(grammar.productions()[3.into()].meta.len(), 1);

        assert_eq!(grammar.productions()[4.into()].assoc, Associativity::Right);
    }
}
