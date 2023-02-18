use std::{
    collections::BTreeMap,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};

use rustemo::{
    index::{
        NonTermIndex, NonTermVec, ProdIndex, ProdVec, SymbolIndex, SymbolVec,
        TermIndex, TermVec,
    },
    Error, Result,
};

use crate::lang::rustemo::RustemoParser;

use self::types::to_snake_case;

use super::lang::rustemo_actions::{
    GrammarSymbol, Imports, ProdMetaDatas, Recognizer, TermMetaDatas,
};

pub(crate) mod builder;
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
    /// Index of EMPTY symbol
    pub empty_index: SymbolIndex,
    /// Index of STOP symbol
    pub stop_index: SymbolIndex,
    /// Index of grammar augmented symbol
    pub augmented_index: SymbolIndex,
    /// Index of augmented symbol for Layout rule if given
    pub augmented_layout_index: Option<SymbolIndex>,
    /// An index of the start symbol. First non-terminal or terminal of the grammar.
    pub start_index: SymbolIndex,
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
    pub action: Option<String>,
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
                production.idx, self.nonterminals[production.nonterminal].name
            )?;
            for assignment in &production.rhs {
                write!(f, "{} ", self.symbol_name(res_symbol(assignment)))?;
            }
            writeln!(f)?;
        }

        writeln!(f, "\n] GRAMMAR")
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            // These two should always be given.
            idx: ProdIndex(usize::MAX),
            nonterminal: NonTermIndex(usize::MAX),

            ntidx: 0,
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
        self.rhs.iter().map(res_symbol).collect()
    }

    /// Returns resolved RHS assignments
    #[inline]
    pub fn rhs_assign(&self) -> Vec<Assignment> {
        self.rhs
            .iter()
            .enumerate()
            .map(|(idx, a)| Assignment {
                name: a.name.clone(),
                symbol: res_symbol(a),
                is_bool: a.is_bool,
                idx,
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

    pub fn to_string(&self, grammar: &Grammar) -> String {
        format!(
            "{}: {}",
            self.nonterminal(grammar).name,
            grammar.symbol_names(self.rhs_symbols()).join(" ")
        )
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
    pub is_bool: bool,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: Option<String>,
    pub symbol: SymbolIndex,
    /// If this assignment is ?= variant. RHS is true if Some.
    pub is_bool: bool,
    /// position in RHS, zero based.
    pub idx: usize,
}

/// Called for Assignment to extract resolved SymbolIndex.
#[inline]
pub(crate) fn res_symbol(assign: &ResolvingAssignment) -> SymbolIndex {
    match assign.symbol {
        ResolvingSymbolIndex::Resolved(index) => index,
        ResolvingSymbolIndex::Resolving(_) => {
            // This shouldn't happen
            panic!("reference {:?} not resolved", assign.symbol);
        }
    }
}

// This can be used at the moment due to conflict with a blankt impl in the core.
// See: https://github.com/rust-lang/rust/issues/50133
// impl<T: AsRef<str>> TryFrom<T> for Grammar {
//     type Error = Error;

//     fn try_from(value: T) -> std::result::Result<Self, Self::Error> {
//        Grammar::from_string(value.as_ref())
//     }
// }

impl FromStr for Grammar {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Grammar::from_string(s)
    }
}

impl Grammar {
    /// Parses given string and constructs a Grammar instance
    fn from_string<G: AsRef<str>>(grammar_str: G) -> Result<Self> {
        RustemoParser::parse(grammar_str.as_ref())?.try_into()
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
        !self.is_empty(symbol)
            && (self.is_nonterm(symbol)
                || self.symbol_to_term(symbol).has_content)
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
    pub fn is_empty(&self, index: SymbolIndex) -> bool {
        index == self.empty_index
    }

    #[inline]
    pub fn production_len(&self, prod: ProdIndex) -> usize {
        self.productions[prod].rhs.len()
    }

    #[inline]
    pub fn production_rhs_symbols(&self, prod: ProdIndex) -> Vec<SymbolIndex> {
        self.productions[prod].rhs.iter().map(res_symbol).collect()
    }

    /// Returns all productions except special AUG and AUGL.
    pub fn productions(&self) -> Vec<&Production> {
        self.productions
            .iter()
            .filter(|&p| {
                let nt_symbol = self.nonterm_to_symbol_index(p.nonterminal);
                nt_symbol != self.augmented_index
                    && self
                        .augmented_layout_index
                        .map_or(true, |li| li != nt_symbol)
            })
            .collect()
    }

    /// Returns all nonterminals except special EMPTY, AUG and AUGL.
    pub fn nonterminals(&self) -> Vec<&NonTerminal> {
        self.nonterminals
            .iter()
            .filter(|&n| {
                let nt_symbol = self.nonterm_to_symbol_index(n.idx);
                nt_symbol != self.empty_index
                    && nt_symbol != self.augmented_index
                    && self
                        .augmented_layout_index
                        .map_or(true, |li| li != nt_symbol)
            })
            .collect()
    }

    #[inline]
    pub fn is_enum(&self, nonterminal: &NonTerminal) -> bool {
        let prods = nonterminal.productions(self);
        prods.iter().filter(|x| x.rhs.len() == 1).count() == prods.len()
    }

    #[inline]
    pub fn has_layout(&self) -> bool {
        self.augmented_layout_index.is_some()
    }

    #[inline]
    pub fn assig_name(
        &self,
        assig: &ResolvingAssignment,
        symbol: SymbolIndex,
    ) -> String {
        to_snake_case(match &assig.name {
            Some(s) => s.clone(),
            None => self.symbol_name(symbol),
        })
    }
}
