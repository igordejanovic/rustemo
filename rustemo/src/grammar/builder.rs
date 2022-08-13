//! Grammar builder. Used to construct the grammar from the parsed AST.

use std::collections::BTreeMap;

use rustemo_rt::index::{
    NonTermIndex, NonTermVec, ProdIndex, ProdVec, TermIndex, TermVec,
};

use crate::{
    grammar::{Grammar, DEFAULT_PRIORITY},
    lang::rustemo_actions::{
        self, ConstVal, GrammarRule, GrammarSymbol, GrammarSymbolRef, PGFile,
        Recognizer, RepetitionOperatorOp, TermMetaDatas,
    },
};

use super::{
    Associativity, NonTerminal, Production, ResolvingAssignment,
    ResolvingSymbolIndex, Terminal,
};

macro_rules! resolving {
    ($name:expr) => {
        ResolvingAssignment {
            name: None,
            symbol: ResolvingSymbolIndex::Resolving(GrammarSymbol::Name($name)),
            is_bool: false,
        }
    };
}

#[derive(Debug)]
pub(crate) struct GrammarBuilder {
    terminals: BTreeMap<String, Terminal>,
    terminals_matches: BTreeMap<String, (String, TermIndex)>,
    nonterminals: BTreeMap<String, NonTerminal>,
    productions: ProdVec<Production>,
    next_term_idx: TermIndex,
    next_nonterm_idx: NonTermIndex,
    next_prod_idx: ProdIndex,
}

impl GrammarBuilder {
    pub fn new() -> Self {
        Self {
            terminals: BTreeMap::new(),
            terminals_matches: BTreeMap::new(),
            nonterminals: BTreeMap::new(),
            productions: ProdVec::new(),
            next_term_idx: TermIndex(0),
            next_nonterm_idx: NonTermIndex(0),
            next_prod_idx: ProdIndex(0),
        }
    }

    fn get_term_idx(&mut self) -> TermIndex {
        let ret = self.next_term_idx;
        self.next_term_idx.0 += 1;
        ret
    }

    fn get_nonterm_idx(&mut self) -> NonTermIndex {
        let ret = self.next_nonterm_idx;
        self.next_nonterm_idx.0 += 1;
        ret
    }

    fn get_prod_idx(&mut self) -> ProdIndex {
        let ret = self.next_prod_idx;
        self.next_prod_idx.0 += 1;
        ret
    }

    pub(crate) fn from_file(mut self, file: PGFile) -> Grammar {
        // Create implicit STOP terminal used to signify the end of the input.
        let term_idx = self.get_term_idx();
        self.terminals.insert(
            "STOP".to_string(),
            Terminal {
                idx: term_idx,
                name: "STOP".to_string(),
                action: None,
                recognizer: None,
                has_content: false,
                prio: DEFAULT_PRIORITY,
                meta: TermMetaDatas::new(),
            },
        );

        // Collect grammar terminals
        if let Some(grammar_terminals) = file.terminal_rules {
            self.collect_terminals(grammar_terminals);
        }

        if let Some(rules) = file.grammar_rules {
            // Extract productions and nonterminals from grammar rules.
            self.extract_productions_and_symbols(rules);
        }

        // Create implicit terminals from string constants.
        self.resolve_inline_terminals_from_productions();

        // Resolve references in productions.
        self.resolve_references();

        let term_len = self.terminals.len();
        let grammar = Grammar {
            imports: file.imports.unwrap_or_default(),
            productions: self.productions,
            empty_index: term_len.into(), // Right after the last terminal
            augmented_index: (term_len + 1).into(), // skip EMPTY
            stop_index: 0.into(),
            term_by_name: self
                .terminals
                .values()
                .map(|t| (t.name.to_string(), t.idx.to_symbol_index()))
                .collect(),
            terminals: {
                let mut terms: TermVec<_> =
                    self.terminals.into_values().collect();
                terms.sort();
                terms
            },
            nonterm_by_name: self
                .nonterminals
                .values()
                .map(|nt| {
                    (nt.name.to_string(), nt.idx.to_symbol_index(term_len))
                })
                .collect(),
            nonterminals: {
                let mut nonterms: NonTermVec<_> =
                    self.nonterminals.into_values().collect();
                nonterms.sort();
                nonterms
            },
        };
        // TODO: Dump only if tracing is used
        log!("{grammar}");
        grammar
    }

    fn collect_terminals(
        &mut self,
        grammar_terminals: Vec<rustemo_actions::Terminal>,
    ) {
        for terminal in grammar_terminals {
            let term_idx = self.get_term_idx();
            self.terminals.insert(
                terminal.name.clone(),
                Terminal {
                    idx: term_idx,
                    name: terminal.name,
                    action: terminal.action,
                    has_content: match &terminal.recognizer {
                        Some(recognizer) => match recognizer {
                            // Terminal has no content only if it is a string match
                            Recognizer::StrConst(_) => false,
                            Recognizer::RegexTerm(_) => true,
                        },
                        None => true,
                    },
                    recognizer: terminal.recognizer,
                    // Extract priority from meta-data
                    prio: match terminal.meta.get("priority") {
                        Some(prio) => match prio {
                            ConstVal::Int(prio) => *prio,
                            _ => unreachable!(),
                        },
                        None => DEFAULT_PRIORITY,
                    },
                    meta: terminal.meta,
                },
            );
        }

        for terminal in self.terminals.values() {
            // Collect each terminal which uses a string match recognizer
            // Those can be used as inline terminals in productions.
            if let Some(Recognizer::StrConst(m)) = &terminal.recognizer {
                self.terminals_matches
                    .insert(m.clone(), (terminal.name.clone(), terminal.idx));
            }
        }
    }

    fn extract_productions_and_symbols(&mut self, rules: Vec<GrammarRule>) {
        // EMPTY non-terminal is implicit
        let nt_idx = self.get_nonterm_idx();
        self.nonterminals.insert(
            "EMPTY".to_string(),
            NonTerminal {
                idx: nt_idx,
                name: "EMPTY".to_string(),
                productions: vec![],
                action: None,
            },
        );

        // Augmented non-terminal and production. by default first rule is
        // starting rule.
        let nt_idx = self.get_nonterm_idx();
        self.nonterminals.insert(
            "AUG".to_string(),
            NonTerminal {
                idx: nt_idx,
                name: "AUG".to_string(),
                productions: vec![ProdIndex(0)],
                action: None,
            },
        );

        // Add augmented S' -> S production
        let prod_idx = self.get_prod_idx();
        self.productions.push(Production {
            idx: prod_idx,
            nonterminal: nt_idx,
            rhs: vec![ResolvingAssignment {
                name: None,
                symbol: ResolvingSymbolIndex::Resolving(GrammarSymbol::Name(
                    rules[0].name.to_string(),
                )),
                is_bool: false,
            }],
            ..Production::default()
        });

        for rule in rules {
            // Create new nonterm index if needed
            let nt_idx;
            if let Some(nonterminal) = self.nonterminals.get(&rule.name) {
                nt_idx = nonterminal.idx;
            } else {
                nt_idx = self.get_nonterm_idx();
            }

            // Gather productions, create indexes. Transform RHS to mark
            // resolving references. Desuggar regex-like references.
            for (prod_ntidx, production) in rule.rhs.into_iter().enumerate() {
                let mut desuggar_productions: Vec<Production> = vec![];
                let prod_idx = self.get_prod_idx();

                let mut new_production = Production {
                    idx: prod_idx,
                    nonterminal: nt_idx,
                    ntidx: prod_ntidx,
                    rhs:
                        production
                            .assignments
                            .into_iter()
                            // Remove EMPTY from production RHS
                            .filter(|assignment| {
                                use rustemo_actions::Assignment;
                                match assignment {
                                    Assignment::GrammarSymbolRef(
                                        GrammarSymbolRef {
                                            gsymbol:
                                                Some(GrammarSymbol::Name(name)),
                                            ..
                                        },
                                    ) if name.as_str() == "EMPTY" => false,
                                    _ => true,
                                }
                            })
                            // Map all RHS elements to Assignments
                            .map(|assignment| {
                                use rustemo_actions::Assignment::*;
                                let is_bool =
                                    matches! { assignment, BoolAssignment(_) };
                                match assignment {
                                    PlainAssignment(mut assign)
                                    | BoolAssignment(mut assign) => {
                                        self.desuggar_regex(
                                            &mut assign.gsymref,
                                            &mut desuggar_productions,
                                        );
                                        ResolvingAssignment {
                                            name: Some(assign.name),
                                            symbol:
                                                ResolvingSymbolIndex::Resolving(
                                                    assign
                                                        .gsymref
                                                        .gsymbol
                                                        .unwrap(),
                                                ),
                                            is_bool,
                                        }
                                    }
                                    GrammarSymbolRef(mut reference) => {
                                        self.desuggar_regex(
                                            &mut reference,
                                            &mut desuggar_productions,
                                        );
                                        ResolvingAssignment {
                                            name: None,
                                            symbol:
                                                ResolvingSymbolIndex::Resolving(
                                                    reference.gsymbol.unwrap(),
                                                ),
                                            is_bool: false,
                                        }
                                    }
                                }
                            })
                            .collect(),
                    meta: production.meta,
                    ..Production::default()
                };

                // Map meta-data to production fields for easier access
                if let Some(meta) = new_production.meta.remove("priority") {
                    new_production.prio = match meta {
                        rustemo_actions::ConstVal::Int(p) => p,
                        _ => panic!("Invalid Const!"),
                    }
                }

                if let Some(kind) = new_production.meta.remove("kind") {
                    new_production.kind = match kind {
                        ConstVal::String(s) => Some(s),
                        _ => None,
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

                self.productions.push(new_production);
                self.productions.extend(desuggar_productions);
                let nonterminal = self
                    .nonterminals
                    .entry(rule.name.clone())
                    .or_insert_with(|| NonTerminal {
                        idx: nt_idx,
                        name: rule.name.clone(),
                        productions: vec![],
                        action: rule.action.clone(),
                    });
                nonterminal.productions.push(prod_idx);
            }
        }
    }

    /// Support for regex-like syntax suggar. E.g: A+, A*, A? and greedy
    /// variants with ! suffix: A*!...
    fn desuggar_regex(
        &mut self,
        gsymref: &mut GrammarSymbolRef,
        productions: &mut Vec<Production>,
    ) {
        fn nt_name(name: &String, rep_op: &RepetitionOperatorOp) -> String {
            format!(
                "{}{}",
                &name,
                match rep_op {
                    RepetitionOperatorOp::ZeroOrMore => "0",
                    RepetitionOperatorOp::ZeroOrMoreGreedy => "0Greedy",
                    RepetitionOperatorOp::OneOrMore => "1",
                    RepetitionOperatorOp::OneOrMoreGreedy => "1Ggreedy",
                    RepetitionOperatorOp::Optional => "Opt",
                    RepetitionOperatorOp::OptionalGreedy => "OptGgreedy",
                }
            )
        }

        if let Some(ref op) = gsymref.repetition_op {
            let modifiers = &op.rep_modifiers;
            let mut modifier = None;
            if let Some(modifiers) = modifiers {
                // For now we only support a separator modifier
                assert!(modifiers.len() == 1);
                modifier = Some(&modifiers[0]);
            }
            let ref_type = match gsymref.gsymbol.as_ref().unwrap() {
                GrammarSymbol::Name(ref name) => name.clone(),
                GrammarSymbol::StrConst(mtch) => {
                    // We have done terminal from strmatch extraction by now so
                    // this unwrap is safe.
                    self.terminals_matches.get(mtch).unwrap().0.clone()
                }
            };

            match op.rep_op {
                RepetitionOperatorOp::ZeroOrMore => {
                    let one_name =
                        nt_name(&ref_type, &RepetitionOperatorOp::OneOrMore);
                    if !self.nonterminals.contains_key(&one_name) {
                        self.create_one(
                            one_name.clone(),
                            &ref_type,
                            &modifier,
                            productions,
                        );
                    }
                    let name = nt_name(&ref_type, &op.rep_op);
                    if !self.nonterminals.contains_key(&name) {
                        self.create_zero(name.clone(), &one_name, productions);
                    }
                    gsymref.gsymbol = Some(GrammarSymbol::Name(name))
                }
                RepetitionOperatorOp::OneOrMore => {
                    let name = nt_name(&ref_type, &op.rep_op);
                    if !self.nonterminals.contains_key(&name) {
                        self.create_one(
                            name.clone(),
                            &ref_type,
                            &modifier,
                            productions,
                        );
                    }
                    gsymref.gsymbol = Some(GrammarSymbol::Name(name))
                }
                RepetitionOperatorOp::Optional => {
                    let name = nt_name(&ref_type, &op.rep_op);
                    if !self.nonterminals.contains_key(&name) {
                        self.create_optional(name.clone(), &ref_type, productions);
                    }
                    gsymref.gsymbol = Some(GrammarSymbol::Name(name))
                }
                RepetitionOperatorOp::OneOrMoreGreedy => todo!(),
                RepetitionOperatorOp::ZeroOrMoreGreedy => todo!(),
                RepetitionOperatorOp::OptionalGreedy => todo!(),
            }
        }
    }

    /// Inline terminals are those created by specifying string match directly
    /// as a part of a production. In such a case we should verify that the
    /// terminal with the same string match is defined and we should resolve
    /// inline instance to the instance provided in "terminals" section.
    ///
    /// Thus, in production you can either reference terminal by name or use the
    /// same string match.
    fn resolve_inline_terminals_from_productions(&mut self) {
        for production in &mut self.productions {
            let production_str = format!("{}", production);
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol
                {
                    if let GrammarSymbol::StrConst(mtch) = symbol {
                        if self.terminals_matches.contains_key(mtch) {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                self.terminals_matches
                                    .get(mtch)
                                    .unwrap()
                                    .1
                                    .to_symbol_index(),
                            );
                        } else {
                            panic!(
                                concat!("terminal \"{}\" used in production \"{}\" ",
                                        "is not defined in the 'terminals' section!."),
                                mtch, production_str)
                        }
                    }
                }
            }
        }
    }

    fn resolve_references(&mut self) {
        // Resolve references.
        for production in &mut self.productions {
            let rhs_len = production.rhs.len();
            let production_str = format!("{}", production);
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol
                {
                    match symbol {
                        GrammarSymbol::Name(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                if let Some(terminal) = self.terminals.get(name)
                                {
                                    terminal.idx.to_symbol_index()
                                } else {
                                    let nt_idx = self
                                        .nonterminals
                                        .get(name)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "unexisting symbol '{}' in production '{}'.",
                                                name, production_str
                                            )
                                        })
                                        .idx;
                                    if rhs_len == 1
                                        && nt_idx == production.nonterminal
                                    {
                                        panic!(
                                            "Infinite recursion on symbol '{}' in production '{}'.",
                                            name, production_str
                                        );
                                    }
                                    nt_idx.to_symbol_index(self.terminals.len())
                                },
                            );
                        }
                        GrammarSymbol::StrConst(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                self.terminals
                                    .get(name)
                                    .unwrap_or_else(|| {
                                        panic!(
                                            "terminal {:?} not created in production '{}'!.",
                                            name, production_str
                                        )
                                    })
                                    .idx
                                    .to_symbol_index(),
                            );
                        }
                    }
                }
            }
        }
    }

    fn create_optional(
        &mut self,
        name: String,
        ref_name: &String,
        productions: &mut Vec<Production>,
    ) {
        let nt_index = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_index,
            name: name.clone(),
            action: None,
            productions: (0..2)
                .map(|idx| {
                    let prod_idx = self.get_prod_idx();
                    if idx == 0 {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_index,
                            ntidx: idx,
                            rhs: vec![resolving!(ref_name.clone())],
                            ..Default::default()
                        });
                    } else {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_index,
                            ntidx: idx,
                            rhs: vec![],
                            ..Default::default()
                        });
                    }
                    prod_idx
                })
                .collect(),
        };
        self.nonterminals.insert(name.clone(), nt);
    }

    fn create_one(
        &mut self,
        name: String,
        ref_name: &String,
        modifier: &Option<&rustemo_actions::RepetitionModifier>,
        productions: &mut Vec<Production>,
    ) {
        let nt_idx = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_idx,
            name: name.clone(),
            action: Some("vec".into()),
            productions: (0..2)
                .map(|idx| {
                    let prod_idx = self.get_prod_idx();
                    if idx == 0 {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_idx,
                            ntidx: idx,
                            rhs: if modifier.is_none() {
                                // without separator
                                vec![
                                    resolving!(name.clone()),
                                    resolving!(ref_name.clone()),
                                ]
                            } else {
                                // with separator.
                                let sep = modifier.unwrap().0.clone();
                                vec![
                                    resolving!(name.clone()),
                                    resolving!(sep),
                                    resolving!(ref_name.clone()),
                                ]
                            },
                            ..Default::default()
                        });
                    } else {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_idx,
                            ntidx: idx,
                            rhs: vec![resolving!(ref_name.clone())],
                            ..Default::default()
                        });
                    }
                    prod_idx
                })
                .collect(),
        };
        self.nonterminals.insert(name.clone(), nt);
    }

    fn create_zero(
        &mut self,
        name: String,
        one_name: &String,
        productions: &mut Vec<Production>,
    ) {
        let nt_idx = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_idx,
            name: name.clone(),
            action: Some("vec".into()),
            productions: (0..2)
                .map(|idx| {
                    let prod_idx = self.get_prod_idx();
                    if idx == 0 {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_idx,
                            ntidx: idx,
                            rhs: vec![resolving!(one_name.clone())],
                            ..Default::default()
                        });
                    } else {
                        productions.push(Production {
                            idx: prod_idx,
                            nonterminal: nt_idx,
                            ntidx: idx,
                            rhs: vec![],
                            ..Default::default()
                        });
                    }
                    prod_idx
                })
                .collect(),
        };
        self.nonterminals.insert(name.clone(), nt);
    }
}
