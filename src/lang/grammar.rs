use indexmap::IndexMap;

use crate::parser::{NonTermIndex, ProdIndex, SymbolIndex, TermIndex};

use super::types::{
    GrammarRule, GrammarSymbol, Imports, PGFile, ProductionMetaDatas, Recognizer, TerminalMetaDatas,
};

#[derive(Debug)]
pub(crate) struct Grammar {
    pub(crate) imports: Option<Imports>,
    pub(crate) productions: Option<Vec<Production>>,
    pub(crate) terminals: Option<Vec<Terminal>>,
    pub(in crate::lang) nonterminals: Option<Vec<NonTerminal>>,
    pub(in crate::lang) nonterm_by_name: IndexMap<String, SymbolIndex>,
    pub(in crate::lang) term_by_name: IndexMap<String, SymbolIndex>,
    // Index of EMPTY symbol
    pub(in crate::lang) empty_index: SymbolIndex,
    // Index of STOP symbol
    pub(in crate::lang) stop_index: SymbolIndex,
    // Index of grammar start symbol
    pub(in crate::lang) start_index: SymbolIndex,
}

#[derive(Debug)]
pub(in crate::lang) struct NonTerminal {
    pub(in crate::lang) idx: NonTermIndex,
    pub(in crate::lang) name: String,
    pub(in crate::lang) productions: Vec<ProdIndex>,
}

#[derive(Debug)]
pub(crate) struct Production {
    pub idx: ProdIndex,
    pub nonterminal: NonTermIndex,
    pub rhs: Vec<Assignment>,
    pub meta: ProductionMetaDatas,
}

#[derive(Debug)]
pub struct Terminal {
    pub idx: TermIndex,
    pub name: String,
    pub action: Option<String>,
    pub recognizer: Option<Recognizer>,
    pub meta: TerminalMetaDatas,
}

#[derive(Debug)]
pub enum ResolvingSymbolIndex {
    Resolved(SymbolIndex),
    Resolving(GrammarSymbol),
}

#[derive(Debug)]
pub(crate) struct Assignment {
    pub(crate) name: Option<String>,
    pub(crate) symbol: ResolvingSymbolIndex,
}

/// Called for Assignment to extract resolved SymbolIndex.
macro_rules! res_symbol {
    ($r:expr) => {
        match $r.symbol {
            ResolvingSymbolIndex::Resolved(index) => index,
            ResolvingSymbolIndex::Resolving(_) => {
                panic!("reference not resolved");
            }
        }
    };
}
pub(crate) use res_symbol;

impl Grammar {
    pub fn from_pgfile(pgfile: PGFile) -> Self {
        let mut terminals: IndexMap<String, Terminal> = IndexMap::new();
        let mut nonterminals: IndexMap<String, NonTerminal> = IndexMap::new();
        let mut productions: Vec<Production> = vec![];

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
            Grammar::extract_productions_and_symbols(rules, &mut nonterminals, &mut productions);
        }

        // TODO: Desugaring. Related to the previous. Desugar repetitions and
        // groups.

        // Collect grammar terminals
        if let Some(grammar_terminals) = pgfile.terminals {
            Grammar::collect_terminals(grammar_terminals, &mut terminals);
        }

        // Create implicit terminals from string constants.
        Grammar::create_terminals_from_productions(&productions, &mut terminals);

        // Resolve references in productions.
        Grammar::resolve_references(&mut productions, &terminals, &nonterminals);

        let term_len = terminals.len();
        Grammar {
            imports: pgfile.imports,
            productions: Some(productions),
            empty_index: terminals.len().into(),
            start_index: (terminals.len() + 1).into(), // skip EMPTY
            stop_index: 0.into(),
            term_by_name: terminals
                .values()
                .map(|t| (t.name.to_string(), t.idx.to_symbol_index()))
                .collect(),
            terminals: if terminals.is_empty() {
                None
            } else {
                Some(terminals.into_values().collect())
            },
            nonterm_by_name: nonterminals
                .values()
                .map(|nt| (nt.name.to_string(), nt.idx.to_symbol_index(term_len)))
                .collect(),
            nonterminals: if nonterminals.is_empty() {
                None
            } else {
                Some(nonterminals.into_values().collect())
            },
        }
    }

    fn extract_productions_and_symbols(
        rules: Vec<GrammarRule>,
        nonterminals: &mut IndexMap<String, NonTerminal>,
        productions: &mut Vec<Production>,
    ) {
        let mut next_nonterm_idx = NonTermIndex(1); // Account for EMPTY and S'
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

        productions.push(Production {
            idx: ProdIndex(0),
            nonterminal: NonTermIndex(1),
            rhs: vec![Assignment {
                name: None,
                symbol: ResolvingSymbolIndex::Resolving(GrammarSymbol::Name(
                    rules[0].name.to_string(),
                )),
            }],
            meta: ProductionMetaDatas::new(),
        });

        for rule in rules {
            // Crate or find non-terminal for the current rule
            nonterminal = nonterminals
                .entry(rule.name.to_string())
                .or_insert_with(|| {
                    next_nonterm_idx.0 += 1;
                    NonTerminal {
                        idx: next_nonterm_idx,
                        name: rule.name.to_string(),
                        productions: vec![],
                    }
                });

            // Gather productions, create indexes. Transform RHS to mark
            // resolving references.
            for production in rule.rhs {
                let new_production = Production {
                    idx: next_prod_idx,
                    nonterminal: nonterminal.idx,
                    rhs: production
                        .assignments
                        .into_iter()
                        .map(|assignment| match assignment {
                            super::types::Assignment::PlainAssignment(assign)
                            | super::types::Assignment::BoolAssignment(assign) => Assignment {
                                name: Some(assign.name),
                                symbol: ResolvingSymbolIndex::Resolving(
                                    assign.gsymref.gsymbol.unwrap(),
                                ),
                            },
                            super::types::Assignment::GSymbolReference(reference) => Assignment {
                                name: None,
                                symbol: ResolvingSymbolIndex::Resolving(reference.gsymbol.unwrap()),
                            },
                        })
                        .collect(),
                    meta: production.meta,
                };
                productions.push(new_production);
                nonterminal.productions.push(next_prod_idx);
                next_prod_idx.0 += 1;
            }
        }
    }

    fn collect_terminals(
        grammar_terminals: Vec<super::types::Terminal>,
        terminals: &mut IndexMap<String, Terminal>,
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
        productions: &Vec<Production>,
        terminals: &mut IndexMap<String, Terminal>,
    ) {
        let mut next_term_idx = TermIndex(terminals.len());
        for production in productions {
            for assign in &production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol {
                    if let GrammarSymbol::StrConst(name) = symbol {
                        if !terminals.contains_key(name) {
                            terminals.insert(
                                name.to_string(),
                                Terminal {
                                    idx: next_term_idx,
                                    name: name.to_string(),
                                    action: None,
                                    recognizer: Some(Recognizer::StrConst(name.to_string())),
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
        productions: &mut Vec<Production>,
        terminals: &IndexMap<String, Terminal>,
        nonterminals: &IndexMap<String, NonTerminal>,
    ) {
        // Resolve references.
        for production in productions {
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol {
                    match symbol {
                        GrammarSymbol::Name(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                if let Some(terminal) = terminals.get(name) {
                                    terminal.idx.to_symbol_index()
                                } else {
                                    nonterminals
                                        .get(name)
                                        .unwrap_or_else(|| panic!("unexisting symbol {:?}.", name))
                                        .idx
                                        .to_symbol_index(terminals.len())
                                },
                            )
                        }
                        GrammarSymbol::StrConst(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                terminals
                                    .get(name)
                                    .unwrap_or_else(|| panic!("terminal {:?} not created!.", name))
                                    .idx
                                    .to_symbol_index(),
                            )
                        }
                    }
                }
            }
        }
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
            self.terminals.as_ref().unwrap()[index.0].name.clone()
        } else {
            self.nonterminals.as_ref().unwrap()[index.0].name.clone()
        }
    }

    pub(crate) fn symbol_indexes(&self, names: &[&str]) -> Vec<SymbolIndex> {
        let mut indexes = Vec::new();
        for name in names {
            indexes.push(self.symbol_index(name))
        }
        indexes
    }

    #[inline]
    pub(crate) fn nonterm_to_symbol(&self, index: NonTermIndex) -> SymbolIndex {
        index.to_symbol_index(self.terminals.as_ref().map_or(0, |t| t.len()))
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

    /// Convert symbol index to non-terminal index. Panics if symbol index is a
    /// terminal index.
    #[inline]
    pub(crate) fn symbol_to_nonterm(&self, index: NonTermIndex) -> SymbolIndex {
        SymbolIndex(index.0 - self.term_len())
    }
}

#[cfg(test)]
mod tests {
    use crate::{lang::parser::GrammarParser, parser::ProdIndex};

    #[test]
    fn create_terminals_1() {
        let grammar = GrammarParser::default().parse(
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
        let grammar = GrammarParser::default().parse(
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
            &["STOP", "third_term", "first_term", "second_term"]
        );
    }

    #[test]
    fn create_terminals_multiple() {
        let grammar = GrammarParser::default().parse(
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
            &["STOP", "third_term", "first_term", "second_term"]
        );
    }

    #[test]
    fn terminals_regex() {
        let grammar = GrammarParser::default().parse(
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
        for (term_name, term_regex) in [("rmatch_term", r#""[^"]+""#), ("more_regex", r#"\d{2,5}"#)]
        {
            assert!(
                match grammar.terminals.as_ref().unwrap()[grammar.term_by_name[term_name].0]
                    .recognizer
                    .as_ref()
                    .unwrap()
                {
                    crate::lang::types::Recognizer::StrConst(_) => false,
                    crate::lang::types::Recognizer::RegExTerm(regex) => regex == term_regex,
                }
            );
        }
    }

    #[test]
    fn nonterminals_productions() {
        let grammar = GrammarParser::default().parse(
            r#"
            S: A "some_term" B | B;
            A: B;
            B: some_term;
            "#
            .into(),
        );
        assert_eq!(grammar.nonterminals.as_ref().unwrap().len(), 5);
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
}
