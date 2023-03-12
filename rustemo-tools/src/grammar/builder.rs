//! Grammar builder. Used to construct the grammar from the parsed AST.
use std::collections::{BTreeMap, BTreeSet};

use rustemo::{
    index::{
        NonTermIndex, NonTermVec, ProdIndex, ProdVec, SymbolIndex, TermIndex,
        TermVec,
    },
    location::ValLoc,
    Error, Result,
};

use crate::{
    grammar::{Grammar, DEFAULT_PRIORITY},
    lang::rustemo_actions::{
        self, ConstVal, File, GrammarRule, GrammarSymbol, GrammarSymbolRef,
        Name, Recognizer, RepetitionOperatorOp, TermMetaDatas,
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
struct GrammarBuilder {
    file: String,
    terminals: BTreeMap<String, Terminal>,
    terminals_matches: BTreeMap<String, (String, TermIndex)>,
    nonterminals: BTreeMap<String, NonTerminal>,
    productions: ProdVec<Production>,
    next_term_idx: TermIndex,
    next_nonterm_idx: NonTermIndex,
    next_prod_idx: ProdIndex,
    start_rule_name: String,
}

impl TryFrom<File> for Grammar {
    type Error = Error;

    fn try_from(value: File) -> Result<Self> {
        GrammarBuilder::new().from_file(value)
    }
}

impl GrammarBuilder {
    fn new() -> Self {
        Self {
            file: "".into(),
            terminals: BTreeMap::new(),
            terminals_matches: BTreeMap::new(),
            nonterminals: BTreeMap::new(),
            productions: ProdVec::new(),
            next_term_idx: TermIndex(0),
            next_nonterm_idx: NonTermIndex(0),
            next_prod_idx: ProdIndex(0),
            start_rule_name: "".into(),
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

    // TODO: Think of a better API which is aligned with conventions
    #[allow(clippy::wrong_self_convention)]
    fn from_file(mut self, file: File) -> Result<Grammar> {
        self.file = file.file;
        // Create implicit STOP terminal used to signify the end of the input.
        let term_idx = self.get_term_idx();
        self.terminals.insert(
            "STOP".to_string(),
            Terminal {
                idx: term_idx,
                name: "STOP".to_string(),
                prio: DEFAULT_PRIORITY,
                meta: TermMetaDatas::new(),
                ..Default::default()
            },
        );

        // Collect grammar terminals
        if let Some(grammar_terminals) = file.terminal_rules {
            self.collect_terminals(grammar_terminals)?;
        }

        if let Some(rules) = file.grammar_rules {
            // Extract productions and nonterminals from grammar rules.
            self.start_rule_name = rules[0].name.as_ref().into();
            self.extract_productions_and_symbols(rules)?;
        }

        // Create implicit terminals from string constants.
        self.resolve_inline_terminals_from_productions()?;

        // Resolve references in productions.
        self.resolve_references()?;

        let term_len = self.terminals.len();
        let grammar = Grammar {
            imports: file.imports.unwrap_or_default(),
            productions: self.productions,
            empty_index: term_len.into(), // Right after the last terminal
            augmented_index: (term_len
                + self.nonterminals.get("AUG").unwrap().idx.0)
                .into(),
            augmented_layout_index: self
                .nonterminals
                .get("AUGL")
                .map(|x| SymbolIndex(term_len + x.idx.0)),
            start_index: (term_len
                + self.nonterminals.get(&self.start_rule_name).unwrap().idx.0)
                .into(),
            stop_index: 0.into(),
            term_by_name: self
                .terminals
                .values()
                .map(|t| (t.name.clone(), t.idx.to_symbol_index()))
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
                .map(|nt| (nt.name.clone(), nt.idx.to_symbol_index(term_len)))
                .collect(),
            nonterminals: {
                let mut nonterms: NonTermVec<_> =
                    self.nonterminals.into_values().collect();
                nonterms.sort();
                nonterms
            },
        };

        mark_reachable_symbols(&grammar);

        // TODO: Dump only if tracing is used
        log!("{grammar}");
        Ok(grammar)
    }

    fn collect_terminals(
        &mut self,
        grammar_terminals: Vec<rustemo_actions::Terminal>,
    ) -> Result<()> {
        for terminal in grammar_terminals {
            let term_idx = self.get_term_idx();
            self.check_identifier(&terminal.name)?;
            self.terminals.insert(
                terminal.name.as_ref().to_string(),
                Terminal {
                    idx: term_idx,
                    name: terminal.name.into(),
                    action: terminal.action.map(|a| a.into()),
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
                    prio: if let Some(ConstVal::Int(prio)) =
                        terminal.meta.get("priority")
                    {
                        prio.into()
                    } else {
                        DEFAULT_PRIORITY
                    },
                    meta: terminal.meta,
                    reachable: false.into(),
                },
            );
        }

        for terminal in self.terminals.values() {
            // Collect each terminal which uses a string match recognizer
            // Those can be used as inline terminals in productions.
            if let Some(Recognizer::StrConst(m)) = &terminal.recognizer {
                self.terminals_matches.insert(
                    (*m).as_ref().into(),
                    (terminal.name.clone(), terminal.idx),
                );
            }
        }
        log!("Terminal matches: {:?}", self.terminals_matches);

        Ok(())
    }

    fn extract_productions_and_symbols(
        &mut self,
        rules: Vec<GrammarRule>,
    ) -> Result<()> {
        // EMPTY non-terminal is implicit
        let nt_idx = self.get_nonterm_idx();
        self.nonterminals.insert(
            "EMPTY".to_string(),
            NonTerminal {
                idx: nt_idx,
                name: "EMPTY".to_string(),
                productions: vec![],
                action: None,
                reachable: false.into(),
            },
        );

        self.create_aug_nt_and_production("AUG", rules[0].name.as_ref());

        let layout_rule = rules
            .iter()
            .find(|r| r.name.as_ref().to_lowercase() == "layout");
        if let Some(layout_rule) = layout_rule {
            self.create_aug_nt_and_production(
                "AUGL",
                layout_rule.name.as_ref(),
            );
        }

        for rule in rules {
            self.check_identifier(&rule.name)?;
            // Create new nonterm index if needed
            let nt_idx;
            if let Some(nonterminal) = self.nonterminals.get(rule.name.as_ref())
            {
                nt_idx = nonterminal.idx;
            } else {
                nt_idx = self.get_nonterm_idx();
            }

            // Gather productions, create indexes. Transform RHS to mark
            // resolving references. Desugar regex-like references.
            for (prod_ntidx, production) in rule.rhs.into_iter().enumerate() {
                let mut desugar_productions: Vec<Production> = vec![];
                let prod_idx = self.get_prod_idx();

                let mut new_production = Production {
                    idx: prod_idx,
                    nonterminal: nt_idx,
                    ntidx: prod_ntidx,
                    rhs: production
                        .assignments
                        .into_iter()
                        // Remove EMPTY from production RHS
                        .filter(|assignment| {
                            use rustemo_actions::Assignment;
                            !matches!(assignment, Assignment::GrammarSymbolRef(
                                        GrammarSymbolRef {
                                            gsymbol:
                                                Some(GrammarSymbol::Name(name)),
                                            ..
                                        },
                                    ) if name.as_ref() == "EMPTY")
                        })
                        // Map all RHS elements to Assignments
                        .map(|assignment| -> Result<ResolvingAssignment> {
                            use rustemo_actions::Assignment::*;
                            let is_bool =
                                matches! { assignment, BoolAssignment(_) };
                            match assignment {
                                PlainAssignment(mut assign)
                                | BoolAssignment(mut assign) => {
                                    self.check_identifier(&assign.name)?;
                                    self.desugar_regex(
                                        &mut assign.gsymref,
                                        &mut desugar_productions,
                                    )?;
                                    Ok(ResolvingAssignment {
                                        name: Some(assign.name),
                                        symbol: ResolvingSymbolIndex::Resolving(
                                            assign.gsymref.gsymbol.unwrap(),
                                        ),
                                        is_bool,
                                    })
                                }
                                GrammarSymbolRef(mut reference) => {
                                    self.desugar_regex(
                                        &mut reference,
                                        &mut desugar_productions,
                                    )?;
                                    Ok(ResolvingAssignment {
                                        name: None,
                                        symbol: ResolvingSymbolIndex::Resolving(
                                            reference.gsymbol.unwrap(),
                                        ),
                                        is_bool: false,
                                    })
                                }
                            }
                        })
                        .collect::<Result<Vec<_>>>()?,
                    meta: production.meta,
                    ..Production::default()
                };

                // Inherit meta-data from Rule.
                for (key, data) in &rule.meta {
                    if !new_production.meta.contains_key(key) {
                        new_production.meta.insert(key.clone(), data.clone());
                    }
                }

                // Map meta-data to production fields for easier access
                if let Some(ConstVal::Int(prio)) =
                    new_production.meta.remove("priority")
                {
                    new_production.prio = prio.into();
                }

                if let Some(ConstVal::String(kind)) =
                    new_production.meta.remove("kind")
                {
                    new_production.kind = Some(kind.into());
                }

                if new_production.meta.remove("left").is_some() {
                    new_production.assoc = Associativity::Left;
                }
                if new_production.meta.remove("right").is_some() {
                    new_production.assoc = Associativity::Right;
                }
                if new_production.meta.remove("nops").is_some() {
                    new_production.nops = true;
                }
                if new_production.meta.remove("nopse").is_some() {
                    new_production.nopse = true;
                }

                self.productions.push(new_production);
                self.productions.extend(desugar_productions);
                let nonterminal = self
                    .nonterminals
                    .entry(rule.name.as_ref().into())
                    .or_insert_with(|| NonTerminal {
                        idx: nt_idx,
                        name: rule.name.as_ref().into(),
                        action: rule.action.as_ref().map(|a| a.as_ref().into()),
                        ..Default::default()
                    });
                nonterminal.productions.push(prod_idx);
            }
        }
        Ok(())
    }

    fn create_aug_nt_and_production(
        &mut self,
        nt_name: &str,
        rhs_rule_name: &str,
    ) {
        // Augmented non-terminal and production.
        let nt_idx = self.get_nonterm_idx();
        let prod_idx = self.get_prod_idx();

        self.nonterminals.insert(
            nt_name.to_string(),
            NonTerminal {
                idx: nt_idx,
                name: nt_name.to_string(),
                productions: vec![prod_idx],
                ..Default::default()
            },
        );

        // Add augmented S' -> S production
        self.productions.push(Production {
            idx: prod_idx,
            nonterminal: nt_idx,
            rhs: vec![ResolvingAssignment {
                name: None,
                symbol: ResolvingSymbolIndex::Resolving(GrammarSymbol::Name(
                    rhs_rule_name.to_string().into(),
                )),
                is_bool: false,
            }],
            ..Production::default()
        });
    }

    /// Support for regex-like syntax sugar. E.g: A+, A*, A? and greedy
    /// variants with ! suffix: A*!...
    fn desugar_regex(
        &mut self,
        gsymref: &mut GrammarSymbolRef,
        productions: &mut Vec<Production>,
    ) -> Result<()> {
        fn nt_name(name: &Name, rep_op: &RepetitionOperatorOp) -> Name {
            Name::new(
                format!(
                    "{}{}",
                    &name,
                    match rep_op {
                        RepetitionOperatorOp::ZeroOrMore => "0",
                        RepetitionOperatorOp::ZeroOrMoreGreedy => "0Greedy",
                        RepetitionOperatorOp::OneOrMore => "1",
                        RepetitionOperatorOp::OneOrMoreGreedy => "1Greedy",
                        RepetitionOperatorOp::Optional => "Opt",
                        RepetitionOperatorOp::OptionalGreedy => "OptGreedy",
                    }
                ),
                name.location,
            )
        }

        if let Some(ref op) = gsymref.repetition_op {
            let modifiers = &op.rep_modifiers;
            let modifier = if let Some(modifiers) = modifiers {
                assert!(
                    modifiers.len() == 1,
                    "Separator modifier is supported only!"
                );
                Some(&modifiers[0])
            } else {
                None
            };
            // TODO: This unwrap may fail in case of production groups use
            // which is still unimplemented but allowed by the grammar.
            let ref_type = match gsymref.gsymbol.as_ref().unwrap() {
                GrammarSymbol::Name(ref name) => name.clone(),
                GrammarSymbol::StrConst(ref mtch) => {
                    if let Some(term) =
                        self.terminals_matches.get(mtch.as_ref())
                    {
                        ValLoc::new(term.0.clone(), mtch.location)
                    } else {
                        return err!(
                            format!(
                                r#"Terminal "{}" is not defined in the terminals section."#,
                                mtch
                            ),
                            Some(self.file.clone()),
                            mtch.location
                        );
                    }
                }
            };

            match op.rep_op {
                RepetitionOperatorOp::ZeroOrMore => {
                    let one_name =
                        nt_name(&ref_type, &RepetitionOperatorOp::OneOrMore);
                    if !self.nonterminals.contains_key(one_name.as_ref()) {
                        self.create_one(
                            one_name.clone(),
                            &ref_type,
                            &modifier,
                            productions,
                        );
                    }
                    let name = nt_name(&ref_type, &op.rep_op);
                    if !self.nonterminals.contains_key(name.as_ref()) {
                        self.create_zero(name.clone(), &one_name, productions);
                    }
                    gsymref.gsymbol = Some(GrammarSymbol::Name(name))
                }
                RepetitionOperatorOp::OneOrMore => {
                    let name = nt_name(&ref_type, &op.rep_op);
                    if !self.nonterminals.contains_key(name.as_ref()) {
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
                    if !self.nonterminals.contains_key(name.as_ref()) {
                        self.create_optional(
                            name.clone(),
                            &ref_type,
                            productions,
                        );
                    }
                    gsymref.gsymbol = Some(GrammarSymbol::Name(name))
                }
                RepetitionOperatorOp::OneOrMoreGreedy => todo!(),
                RepetitionOperatorOp::ZeroOrMoreGreedy => todo!(),
                RepetitionOperatorOp::OptionalGreedy => todo!(),
            }
        }
        Ok(())
    }

    /// Inline terminals are those created by specifying string match directly
    /// as a part of a production. In such a case we should verify that the
    /// terminal with the same string match is defined and we should resolve
    /// inline instance to the instance provided in "terminals" section.
    ///
    /// Thus, in production you can either reference terminal by name or use the
    /// same string match.
    fn resolve_inline_terminals_from_productions(&mut self) -> Result<()> {
        for production in &mut self.productions {
            let production_str = format!("{}", production);
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(
                    GrammarSymbol::StrConst(mtch),
                ) = &assign.symbol
                {
                    if self.terminals_matches.contains_key(mtch.as_ref()) {
                        assign.symbol = ResolvingSymbolIndex::Resolved(
                            self.terminals_matches
                                .get(mtch.as_ref())
                                .unwrap()
                                .1
                                .to_symbol_index(),
                        );
                    } else {
                        err!(
                            format!(
                                concat!(
                                "Terminal \"{}\" used in production \"{}\" ",
                                "is not defined in the 'terminals' section."
                            ),
                                mtch, production_str
                            ),
                            Some(self.file.clone()),
                            mtch.location
                        )?
                    }
                }
            }
        }
        Ok(())
    }

    fn resolve_references(&mut self) -> Result<()> {
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
                                if let Some(terminal) =
                                    self.terminals.get(name.as_ref())
                                {
                                    terminal.idx.to_symbol_index()
                                } else {
                                    let nt_idx = self
                                        .nonterminals
                                        .get(name.as_ref())
                                        .ok_or_else(|| {
                                            let r: Result<()> = err!(format!("Unexisting symbol '{}' in production '{}'.",
                                                         name, production_str),
                                                 Some(self.file.clone()), name.location);
                                            r.unwrap_err()
                                        })?.idx;
                                    if rhs_len == 1
                                        && nt_idx == production.nonterminal
                                    {
                                        err!(format!("Infinite recursion on symbol '{}' in production '{}'.",
                                            name, production_str),
                                             Some(self.file.clone()), name.location)?;
                                    }
                                    nt_idx.to_symbol_index(self.terminals.len())
                                },
                            );
                        }
                        GrammarSymbol::StrConst(name) => {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                self.terminals
                                    .get(name.as_ref())
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
        Ok(())
    }

    fn create_optional(
        &mut self,
        name: Name,
        ref_name: &Name,
        productions: &mut Vec<Production>,
    ) {
        let nt_index = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_index,
            name: name.as_ref().clone(),
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
            ..Default::default()
        };
        self.nonterminals.insert(name.into(), nt);
    }

    fn create_one(
        &mut self,
        name: Name,
        ref_name: &Name,
        modifier: &Option<&rustemo_actions::RepetitionModifier>,
        productions: &mut Vec<Production>,
    ) {
        let nt_idx = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_idx,
            name: name.as_ref().clone(),
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
                                let sep = modifier.unwrap().clone();
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
            reachable: false.into(),
        };
        self.nonterminals.insert(name.into(), nt);
    }

    fn create_zero(
        &mut self,
        name: Name,
        one_name: &Name,
        productions: &mut Vec<Production>,
    ) {
        let nt_idx = self.get_nonterm_idx();
        let nt = NonTerminal {
            idx: nt_idx,
            name: name.as_ref().clone(),
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
            reachable: false.into(),
        };
        self.nonterminals.insert(name.into(), nt);
    }

    fn check_identifier(&self, name: &ValLoc<String>) -> Result<()> {
        let result = syn::parse_str::<syn::Ident>(name.as_ref());
        if result.is_err() {
            err!(
                format!("Can't use '{}' as a valid Rust identifier.", &name),
                Some(self.file.clone()),
                name.location
            )?
        }
        Ok(())
    }
}

fn mark_reachable_symbols(grammar: &Grammar) {
    let mut visited = BTreeSet::<ProdIndex>::new();

    fn mark_reachable(
        grammar: &Grammar,
        nonterm: &NonTerminal,
        visited: &mut BTreeSet<ProdIndex>,
    ) {
        nonterm.reachable.set(true);
        for prod in &nonterm.productions {
            if visited.contains(prod) {
                continue;
            }
            visited.insert(*prod);
            for symbol in grammar.productions[*prod].rhs_symbols() {
                if grammar.is_nonterm(symbol) {
                    mark_reachable(
                        grammar,
                        grammar.symbol_to_nonterm(symbol),
                        visited,
                    )
                } else {
                    grammar.symbol_to_term(symbol).reachable.set(true);
                }
            }
        }
    }

    mark_reachable(
        grammar,
        &grammar.nonterminals
            [grammar.symbol_to_nonterm_index(grammar.start_index)],
        &mut visited,
    );
}
