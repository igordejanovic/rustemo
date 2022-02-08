use crate::{common::Location, parser::Action, index::TermIndex};
use std::hash::Hash;

pub(crate) type Priority = u8;

#[derive(Debug)]
struct Recognizer;

#[derive(Debug)]
pub enum Symbol<'a> {
    Terminal(Terminal),
    NonTerminal(NonTerminal<'a>),
}

impl<'a> Hash for Symbol<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            Symbol::Terminal(t) => t.name.hash(state),
            Symbol::NonTerminal(n) => n.name.hash(state),
        }
    }
}

/// Represent a terminal symbol of the grammar.
#[derive(Debug, Default)]
pub struct Terminal {
    pub name: &'static str,
    pub fqn: &'static str,
    pub location: Option<Location>,

    /// Priority used for lexical disambiguation.
    pub prior: Priority,

    /// Used for scanning optimization. If this terminal is `finish` no other
    /// recognizers will be checked if this succeeds. If not provided in the
    /// grammar implicit rules will be used during table construction.
    pub finish: bool,

    /// Prefer this recognizer in case of multiple recognizers match at the same
    /// place and implicit disambiguation doesn't resolve.
    pub prefer: bool,
    // /// Should dynamic disambiguation be called to resolve conflict involving
    // /// this terminal.
    // pub dynamic: bool,

    // /// `true` if this Terminal represents keyword. `false` by default.
    // pub keyword: bool,
}

// impl Symbol for Terminal {
//     fn as_symbolbase(&self) -> &SymbolBase {
//         &self.base
//     }
// }

#[derive(Debug, Default)]
pub struct NonTerminal<'a> {
    pub name: &'a str,
    pub fqn: &'a str,
    pub location: Option<Location>,

    /// Productions which define this non-terminal rule.
    pub productions: Vec<&'a Production<'a>>,
}

#[derive(Debug)]
pub struct Production<'a> {
    pub id: usize,
    pub lhs: &'a NonTerminal<'a>,
    pub rhs: Vec<&'a Symbol<'a>>,
}

pub type TerminalInfos<const T: usize> = [TerminalInfo; T];
pub type TerminalsState<const T: usize, const S: usize> = [[Option<usize>; T]; S];
pub type Actions<const T: usize, const S: usize> = [[Action; T]; S];
pub type Gotos<const N: usize, const S: usize> = [[Option<usize>; N]; S];

#[derive(Debug)]
pub struct TerminalInfo {
    pub id: TermIndex,
    pub name: &'static str,
    pub location: Option<Location>,
}

#[derive(Debug)]
pub struct NonTerminalInfo {
    pub id: usize,
    pub name: &'static str,
    pub location: Option<Location>,
    #[cfg(debug_assertions)]
    pub production_str: &'static str,
}

