use std::{
    collections::BTreeMap,
    fmt::Display,
    hash::{Hash, Hasher},
};

use convert_case::{Case, Casing};
use rustemo_rt::{
    index::{
        NonTermIndex, NonTermVec, ProdIndex, ProdVec, SymbolIndex, SymbolVec,
        TermIndex, TermVec,
    },
    log,
};

use crate::{error::Result, lang::rustemo::RustemoParser};

use super::lang::rustemo_actions::{
    self, Const, GrammarRule, GrammarSymbol, Imports, PGFile, ProdMetaDatas,
    Recognizer, TermMetaDatas,
};

#[cfg(test)]
mod tests;
pub(crate) mod types;

#[derive(Debug)]
pub struct Grammar {
    pub imports: Imports,
    pub productions: ProdVec<Production>,
    pub terminals: TermVec<Terminal>,
    pub nonterminals: NonTermVec<NonTerminal>,
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

    /// Terminal will carry content if it is a non-constant match (e.g. a regex
    /// or a custom recognizer).
    pub has_content: bool,

    pub prio: Priority,
    pub meta: TermMetaDatas,
}
grammar_elem!(Terminal);

#[derive(Debug)]
pub struct NonTerminal {
    pub idx: NonTermIndex,
    pub name: String,
    pub productions: Vec<ProdIndex>,
}
grammar_elem!(NonTerminal);

impl NonTerminal {
    #[inline]
    pub fn productions<'a>(&self, grammar: &'a Grammar) -> Vec<&'a Production> {
        self.productions
            .iter()
            .map(|&idx| &grammar.productions[idx])
            .collect()
    }
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nGRAMMAR [")?;
        writeln!(f, "\nTerminals:")?;
        for terminal in &self.terminals {
            writeln!(f, "{}. {}", terminal.idx, terminal.name)?;
        }
        writeln!(f, "\nNonTerminals:")?;
        for nonterminal in &self.nonterminals {
            writeln!(
                f,
                "{} ({}). {}",
                nonterminal.idx,
                self.nonterm_to_symbol_index(nonterminal.idx),
                nonterminal.name
            )?;
        }
        writeln!(f, "\nProductions:")?;
        for production in &self.productions {
            write!(
                f,
                "{}. {}: ",
                production.idx,
                self.nonterminals[production.nonterminal].name
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
    pub ntidx: usize,
    pub kind: Option<String>,
    pub rhs: Vec<ResolvingAssignment>,
    pub assoc: Associativity,
    pub prio: Priority,
    pub dynamic: bool,
    pub nops: bool,
    pub nopse: bool,
    pub meta: ProdMetaDatas,
}
grammar_elem!(Production);

impl Default for Production {
    fn default() -> Self {
        Self {
            idx: Default::default(),
            nonterminal: Default::default(),
            ntidx: Default::default(),
            kind: None,
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
    #[inline]
    pub fn rhs_symbols(&self) -> Vec<SymbolIndex> {
        self.rhs.iter().map(|a| res_symbol(a)).collect()
    }

    /// Returns resolved RHS assignments
    #[inline]
    pub fn rhs_assign(&self) -> Vec<Assignment> {
        self.rhs
            .iter()
            .map(|a| Assignment {
                name: a.name.clone(),
                symbol: res_symbol(a),
            })
            .collect()
    }

    /// Returns RHS assignment which has some content (i.e. non-terminals and
    /// non-constant terminals).
    pub fn rhs_with_content(&self, grammar: &Grammar) -> Vec<Assignment> {
        self.rhs_assign()
            .into_iter()
            .filter_map(|a| {
                if grammar.symbol_has_content(a.symbol) {
                    Some(a)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn rhs_symbol(&self, pos: usize) -> SymbolIndex {
        res_symbol(&self.rhs[pos])
    }

    #[inline]
    pub fn nonterminal<'a>(&self, grammar: &'a Grammar) -> &'a NonTerminal {
        &grammar.nonterminals[self.nonterminal]
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.idx)?;
        for assign in &self.rhs {
            if assign.name.is_some() {
                write!(f, " {} ", assign.name.as_ref().unwrap())?;
            } else {
                let s = match &assign.symbol {
                    ResolvingSymbolIndex::Resolved(symbol) => {
                        format!("{}", symbol)
                    }
                    ResolvingSymbolIndex::Resolving(symbol) => match symbol {
                        GrammarSymbol::Name(name) => name.into(),
                        GrammarSymbol::StrConst(mtch) => {
                            format!("\"{}\"", mtch)
                        }
                    },
                };
                write!(f, " {} ", s)?;
            }
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub enum ResolvingSymbolIndex {
    Resolved(SymbolIndex),
    Resolving(GrammarSymbol),
}

#[derive(Debug)]
pub struct ResolvingAssignment {
    pub name: Option<String>,
    pub symbol: ResolvingSymbolIndex,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: Option<String>,
    pub symbol: SymbolIndex,
}

/// Called for Assignment to extract resolved SymbolIndex.
#[inline]
pub(in crate) fn res_symbol(assign: &ResolvingAssignment) -> SymbolIndex {
    match assign.symbol {
        ResolvingSymbolIndex::Resolved(index) => index,
        ResolvingSymbolIndex::Resolving(_) => {
            // This shouldn't happen
            panic!("reference {:?} not resolved", assign.symbol);
        }
    }
}

impl Grammar {
    /// Parses given string and constructs a Grammar instance
    pub fn from_string<G: AsRef<str>>(grammar_str: G) -> Result<Self> {
        Ok(Self::from_pgfile(RustemoParser::parse_str(
            grammar_str.as_ref(),
        )?))
    }

    /// Parses given file and constructs a Grammar instance
    /// FIXME: Return/move owned string from file content.
    // pub fn from_file<F: AsRef<Path>>(file: F) -> Result<Self> {
    //     use crate::rustemo_types::{NonTerminal, Symbol};
    //     if let Symbol::NonTerminal(NonTerminal::PGFile(pgfile)) =
    //         RustemoParser::parse_file(file)?
    //     {
    //         Ok(Self::from_pgfile(pgfile))
    //     } else {
    //         panic!("Invalid symbol from grammar parse!")
    //     }
    // }

    fn from_pgfile(pgfile: PGFile) -> Self {
        let mut terminals: BTreeMap<String, Terminal> = BTreeMap::new();
        let mut terminals_matches: BTreeMap<String, &Terminal> =
            BTreeMap::new();
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
                has_content: false,
                prio: DEFAULT_PRIORITY,
                meta: TermMetaDatas::new(),
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
            Grammar::collect_terminals(
                grammar_terminals,
                &mut terminals,
                &mut terminals_matches,
            );
        }

        // Create implicit terminals from string constants.
        Grammar::resolve_inline_terminals_from_productions(
            &mut productions,
            &terminals_matches,
        );

        // Resolve references in productions.
        Grammar::resolve_references(
            &mut productions,
            &terminals,
            &nonterminals,
        );

        let term_len = terminals.len();
        let grammar = Grammar {
            imports: pgfile.imports.unwrap_or_default(),
            productions,
            empty_index: terminals.len().into(), // Right after the last terminal
            augmented_index: (terminals.len() + 1).into(), // skip EMPTY
            stop_index: 0.into(),
            term_by_name: terminals
                .values()
                .map(|t| (t.name.to_string(), t.idx.to_symbol_index()))
                .collect(),
            terminals: {
                let mut terms: TermVec<_> = terminals.into_values().collect();
                terms.sort();
                terms
            },
            nonterm_by_name: nonterminals
                .values()
                .map(|nt| {
                    (nt.name.to_string(), nt.idx.to_symbol_index(term_len))
                })
                .collect(),
            nonterminals: {
                let mut nonterms: NonTermVec<_> =
                    nonterminals.into_values().collect();
                nonterms.sort();
                nonterms
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
            "AUG".to_string(),
            NonTerminal {
                idx: NonTermIndex(1),
                name: "AUG".to_string(),
                productions: vec![ProdIndex(0)],
            },
        );

        // Add augmented S' -> S production
        productions.push(Production {
            idx: ProdIndex(0),
            nonterminal: NonTermIndex(1),
            rhs: vec![ResolvingAssignment {
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
                    rhs:
                        production
                            .assignments
                            .into_iter()
                            // Remove EMPTY from production RHS
                            .filter(|assignment| {
                                use rustemo_actions::{
                                    Assignment, GrammarSymbolRef,
                                };
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
                                match assignment {
                                    PlainAssignment(assign)
                                    | BoolAssignment(assign) => {
                                        ResolvingAssignment {
                                            name: Some(assign.name),
                                            symbol:
                                                ResolvingSymbolIndex::Resolving(
                                                    assign
                                                        .gsymref
                                                        .gsymbol
                                                        .unwrap(),
                                                ),
                                        }
                                    }
                                    GrammarSymbolRef(reference) => {
                                        ResolvingAssignment {
                                            name: None,
                                            symbol:
                                                ResolvingSymbolIndex::Resolving(
                                                    reference.gsymbol.unwrap(),
                                                ),
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
                        rustemo_actions::Const::Int(p) => p,
                        _ => panic!("Invalid Const!"),
                    }
                }

                if let Some(kind) = new_production.meta.remove("kind") {
                    new_production.kind = match kind {
                        Const::String(s) => Some(s),
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

                new_production.ntidx = nonterminal.productions.len();
                productions.push(new_production);
                nonterminal.productions.push(next_prod_idx);
                next_prod_idx.0 += 1;
            }
        }
    }

    fn collect_terminals<'a>(
        grammar_terminals: Vec<rustemo_actions::Terminal>,
        terminals: &'a mut BTreeMap<String, Terminal>,
        terminals_matches: &mut BTreeMap<String, &'a Terminal>,
    ) {
        let mut next_term_idx = TermIndex(1); // Account for STOP terminal
        for terminal in grammar_terminals {
            terminals.insert(
                terminal.name.clone(),
                Terminal {
                    idx: next_term_idx,
                    name: terminal.name,
                    action: terminal.action,
                    has_content: match &terminal.recognizer {
                        Some(recognizer) => match recognizer {
                            // Terminal has no content only if it is a string match
                            Recognizer::StrConst(_) => false,
                            Recognizer::RegExTerm(_) => true,
                        },
                        None => true,
                    },
                    recognizer: terminal.recognizer,
                    // Extract priority from meta-data
                    prio: match terminal.meta.get("priority") {
                        Some(prio) => match prio {
                            Const::Int(prio) => *prio,
                            _ => unreachable!(),
                        },
                        None => DEFAULT_PRIORITY,
                    },
                    meta: terminal.meta,
                },
            );
            next_term_idx.0 += 1;
        }

        for terminal in terminals.values() {
            // Collect each terminal which uses a string match recognizer
            // Those can be used as inline terminals in productions.
            if let Some(Recognizer::StrConst(m)) = &terminal.recognizer {
                terminals_matches.insert(m.clone(), &terminal);
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
    fn resolve_inline_terminals_from_productions(
        productions: &mut ProdVec<Production>,
        terminals_matches: &BTreeMap<String, &Terminal>,
    ) {
        for production in productions {
            let production_str = format!("{}", production);
            for assign in &mut production.rhs {
                if let ResolvingSymbolIndex::Resolving(symbol) = &assign.symbol
                {
                    if let GrammarSymbol::StrConst(mtch) = symbol {
                        if terminals_matches.contains_key(mtch) {
                            assign.symbol = ResolvingSymbolIndex::Resolved(
                                terminals_matches
                                    .get(mtch)
                                    .unwrap()
                                    .idx
                                    .to_symbol_index(),
                            );
                        } else {
                            panic!(
                                concat!("terminal \"{}\" used in production \"{:?}\" ",
                                        "is not defined in the 'terminals' section!."),
                                mtch, production_str)
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
        TermVec(vec![default; self.terminals.len()])
    }

    pub(crate) fn new_nontermvec<T: Clone>(&self, default: T) -> NonTermVec<T> {
        NonTermVec(vec![default; self.nonterminals.len()])
    }

    pub fn symbol_index(&self, name: &str) -> SymbolIndex {
        *self.term_by_name.get(name).unwrap_or_else(|| {
            self.nonterm_by_name.get(name).unwrap_or_else(|| {
                panic!("No Symbol by name {:?}", name);
            })
        })
    }

    pub fn symbol_name(&self, index: SymbolIndex) -> String {
        if index.0 < self.terminals.len() {
            self.symbol_to_term(index).name.clone()
        } else {
            self.symbol_to_nonterm(index).name.clone()
        }
    }

    /// If this symbol is either a non-terminal of a terminal with a content.
    /// I.e. not a constant match terminal (keyword, punctuation...)
    #[inline]
    pub fn symbol_has_content(&self, symbol: SymbolIndex) -> bool {
        self.is_nonterm(symbol) || self.symbol_to_term(symbol).has_content
    }

    pub fn symbol_indexes(&self, names: &[&str]) -> SymbolVec<SymbolIndex> {
        let mut indexes = SymbolVec::new();
        for name in names {
            indexes.push(self.symbol_index(name))
        }
        indexes
    }

    #[inline]
    pub fn symbol_names<T>(&self, indexes: T) -> Vec<String>
    where
        T: IntoIterator<Item = SymbolIndex>,
    {
        indexes.into_iter().map(|i| self.symbol_name(i)).collect()
    }

    #[inline]
    pub fn term_to_symbol_index(&self, index: TermIndex) -> SymbolIndex {
        SymbolIndex(index.0)
    }

    /// Convert symbol index to terminal index.
    #[inline]
    pub fn symbol_to_term_index(&self, index: SymbolIndex) -> TermIndex {
        TermIndex(index.0)
    }

    /// Convert symbol index to terminal
    #[inline]
    pub fn symbol_to_term(&self, index: SymbolIndex) -> &Terminal {
        &self.terminals[self.symbol_to_term_index(index)]
    }

    #[inline]
    pub fn nonterm_to_symbol_index(&self, index: NonTermIndex) -> SymbolIndex {
        SymbolIndex(index.0 + self.terminals.len())
    }

    /// Convert symbol index to non-terminal index. Panics if symbol index is a
    /// terminal index.
    #[inline]
    pub fn symbol_to_nonterm_index(&self, index: SymbolIndex) -> NonTermIndex {
        NonTermIndex(index.0.checked_sub(self.terminals.len()).unwrap())
    }

    /// Convert symbol index to non-terminal. Panics if symbol index is a
    /// terminal index.
    #[inline]
    pub fn symbol_to_nonterm(&self, index: SymbolIndex) -> &NonTerminal {
        &self.nonterminals
            [NonTermIndex(index.0.checked_sub(self.terminals.len()).unwrap())]
    }

    /// Convert symbol index to non-terminal. Panics if symbol index is a
    /// terminal index.
    // #[inline]
    // pub fn symbol_to_nonterm(&self, index: SymbolIndex) -> NonTermIndex {
    //     NonTermIndex(index.0.checked_sub(self.term_len()).unwrap())
    // }

    #[inline]
    pub fn is_nonterm(&self, index: SymbolIndex) -> bool {
        index.0 >= self.terminals.len()
    }

    #[inline]
    pub fn is_term(&self, index: SymbolIndex) -> bool {
        index.0 < self.terminals.len()
    }

    #[inline]
    pub fn production_len(&self, prod: ProdIndex) -> usize {
        self.productions[prod].rhs.len()
    }

    #[inline]
    pub fn production_rhs_symbols(&self, prod: ProdIndex) -> Vec<SymbolIndex> {
        self.productions[prod]
            .rhs
            .iter()
            .map(|assgn| res_symbol(assgn))
            .collect()
    }

    #[inline]
    pub fn is_enum(&self, nonterminal: &NonTerminal) -> bool {
        let prods = nonterminal.productions(self);
        prods.iter().filter(|x| x.rhs.len() == 1).count() == prods.len()
    }

    #[inline]
    pub fn assig_name(
        &self,
        assig: &ResolvingAssignment,
        symbol: SymbolIndex,
    ) -> String {
        match &assig.name {
            Some(s) => s.clone(),
            None => self.symbol_name(symbol),
        }
        .to_case(Case::Snake)
    }
}
