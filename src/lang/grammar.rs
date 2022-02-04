use indexmap::IndexMap;

use crate::parser::{NonTermIndex, ProdIndex, SymbolIndex, TermIndex};

use super::types::{
    GrammarSymbol, Imports, PGFile, ProductionMetaDatas, Recognizer, TerminalMetaDatas,
};

#[derive(Debug)]
pub(in crate::lang) struct Grammar {
    imports: Option<Imports>,
    productions: Option<Vec<Production>>,
    terminals: Option<Vec<Terminal>>,
    nonterminals: Option<Vec<NonTerminal>>,
    // nonterminals: Vec<NonTerminalRules>,
    // symbol_by_name: HashMap<String, &'a Symbol<'a>>,
    // first_set: HashMap<NonTerminal<'a>, HashSet<&'a Terminal>>,
    // start_symbol: Option<&'a NonTerminal<'a>>,
}

#[derive(Debug)]
pub(in crate::lang) struct NonTerminal {
    idx: NonTermIndex,
    name: String,
    productions: Vec<ProdIndex>,
}

#[derive(Debug)]
pub struct Production {
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
pub struct Assignment {
    name: Option<String>,
    symbol: ResolvingSymbolIndex,
}

impl Grammar {
    pub fn from_pgfile(pgfile: PGFile) -> Self {
        let mut nonterminals: IndexMap<String, NonTerminal> = IndexMap::new();
        let mut terminals: IndexMap<String, Terminal> = IndexMap::new();
        let mut productions = vec![];

        if let Some(rules) = pgfile.rules {
            let mut next_nonterm_idx = NonTermIndex(0);
            let mut next_prod_idx = ProdIndex(0);
            let mut nonterminal;

            for rule in rules {
                // Crate or find non-terminal for the current rule
                if !nonterminals.contains_key(&rule.name) {
                    nonterminals.insert(
                        rule.name.to_string(),
                        NonTerminal {
                            idx: next_nonterm_idx,
                            name: rule.name.to_string(),
                            productions: vec![],
                        },
                    );
                    next_nonterm_idx.0 += 1;
                }
                nonterminal = &mut nonterminals[&rule.name];

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
                                super::types::Assignment::PlainAssignment(assign) => Assignment {
                                    name: Some(assign.name),
                                    symbol: ResolvingSymbolIndex::Resolving(
                                        assign.gsymref.gsymbol.unwrap(),
                                    ),
                                },
                                super::types::Assignment::BoolAssignment(assign) => Assignment {
                                    name: Some(assign.name),
                                    symbol: ResolvingSymbolIndex::Resolving(
                                        assign.gsymref.gsymbol.unwrap(),
                                    ),
                                },
                                super::types::Assignment::GSymbolReference(reference) => {
                                    Assignment {
                                        name: None,
                                        symbol: ResolvingSymbolIndex::Resolving(
                                            reference.gsymbol.unwrap(),
                                        ),
                                    }
                                }
                            })
                            .collect(),
                        meta: production.meta,
                    };
                    productions.push(new_production);
                    nonterminal.productions.push(next_prod_idx);
                    next_prod_idx.0 += 1;
                    // 3. TODO: Desugaring. Related to the previous. Desugar repetitions and
                    // groups.

                    // for production in desugar_production(production) {

                    // }
                }
            }
        }

        // Collect grammar terminals
        let mut next_term_idx = TermIndex(0);
        if let Some(grammar_terminals) = pgfile.terminals {
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

        // Create new terminals from str consts
        for production in &mut productions {
            for assign in &mut production.rhs {
                match &assign.symbol {
                    ResolvingSymbolIndex::Resolved(_) => {}
                    ResolvingSymbolIndex::Resolving(symbol) => match symbol {
                        GrammarSymbol::StrConst(name) => {
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
                        GrammarSymbol::Name(_) => {}
                    },
                }
            }
        }

        // Resolve references.
        for production in &mut productions {
            for assign in &mut production.rhs {
                match &assign.symbol {
                    ResolvingSymbolIndex::Resolving(symbol) => match symbol {
                        GrammarSymbol::Name(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                if let Some(terminal) = terminals.get(name) {
                                    terminal.idx.into()
                                } else {
                                    nonterminals
                                        .get(name)
                                        .unwrap_or_else(|| {
                                            panic!("unexisting symbol {:?}.", name)
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
                                        panic!("terminal {:?} not created!.", name)
                                    })
                                    .idx
                                    .into(),
                            )
                        }
                    },
                    ResolvingSymbolIndex::Resolved(_) => {}
                }
            }
        }

        Grammar {
            imports: pgfile.imports,
            productions: Some(productions),
            terminals: if terminals.is_empty() {
                None
            } else {
                Some(terminals.into_values().collect())
            },
            nonterminals: if nonterminals.is_empty() {
                None
            } else {
                Some(nonterminals.into_values().collect())
            },
        }
    }
}

// fn desugar_production(production: super::types::Production) -> impl Iterator<Item=Production> {
//     production.assignments
//               .iter()
//               .filter(|assignment| match assignment {
//                   super::types::Assignment::PlainAssignment(_) => todo!(),
//                   super::types::Assignment::BoolAssignment(_) => todo!(),
//                   super::types::Assignment::GSymbolReference(_) => todo!(),
//               }).collect();
//     todo!()
// }

// impl<'a> Grammar<'a> {
//     fn new(
//         productions: Vec<Production>,
//         terminals: Vec<Terminal>,
//         nonterminals: Vec<NonTerminal<'a>>,
//         start_symbol: Option<&'a NonTerminal<'a>>,
//     ) -> Grammar<'a> {
//         return Grammar {
//         //    productions,
//             terminals,
//             nonterminals,
//             first_set: HashMap::new(),
//             start_symbol,
//         };
//     }

//     fn add_terminal(&'a mut self, fqn: &str) -> &mut Self {
//         let t = Terminal {
//             name: fqn.split('.').last().unwrap().to_string(),
//             fqn: fqn.to_string(),
//             ..Terminal::default()
//         };
//         self.terminals.push(t);
//         self.symbol_by_name[fqn] = &Symbol::Terminal(t);
//         self
//     }

//     fn add_nonterminal(&'a mut self, fqn: String) -> &mut Self {
//         self.nonterminals.push(NonTerminal {
//             name: fqn.split('.').last().unwrap().to_string(),
//             fqn,
//             ..NonTerminal::default()
//         });
//         self
//     }

//     fn add_production(&'a mut self, nonterm_fqn: &str, rhs_names: &[&str]) -> &mut Self {
//         let rhs: Vec<&Symbol> = Vec::new();
//         for symbol_ref in rhs_names {
//             rhs.push(self.symbol_by_name(symbol_ref))
//         }
//        self
//     }

//     fn symbol_by_name(&self, name: &str) -> &Symbol {
//         self.symbol_by_name[name]
//     }

//     /// Calculate and update grammar first sets.
//     ///
//     /// The Dragon book, p 245.
//     ///
//     /// Define $FIRST(\alpha)$ where $\alpha$ is any string of grammar
//     /// symbols, to be the set of terminals that begin strings derived from
//     /// $\alpha$. If $\alpha \overset{*}{\Rightarrow} \epsilon$ then
//     /// $\epsilon$ is also in $FIRST(\alpha)$.
//     fn first(&mut self) -> () {
//         // 1. Initialize firsts set for every terminal to a set with
//         //    the terminal being its sole member.
//         // 2. Initialize firsts set for every non-terminal to an empty set.
//         // 3.
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::grammar::Grammar;

//     #[test]
//     fn test_create_grammar() {
//         let grammar = Grammar::new(vec![], vec![], vec![], None);
//     }

//     #[test]
//     fn test_grammar_first_sets() {
//         let grammar = Grammar::new(vec![Production::new()], vec![], vec![], None);

//         grammar.first_set();
//     }

//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
