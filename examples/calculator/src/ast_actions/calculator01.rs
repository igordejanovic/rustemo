/// Generated by rustemo. Do not edit manually!
use regex::Regex;
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, fmt::Debug};
use rustemo_rt::lexer::{Lexer, Token};
use rustemo_rt::parser::Parser;
use rustemo_rt::builder::Builder;
use rustemo_rt::Result;
use rustemo_rt::lr::lexer::{
    LRStringLexer, LRContext, LexerDefinition, RecognizerIterator,
};
use rustemo_rt::lr::builder::LRBuilder;
use rustemo_rt::lr::parser::{LRParser, ParserDefinition};
use rustemo_rt::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
use rustemo_rt::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
use rustemo_rt::grammar::{TerminalInfo, TerminalInfos, TerminalsState};
use rustemo_rt::debug::{log, logn};
use super::calculator01_actions;
const TERMINAL_NO: usize = 6usize;
const NONTERMINAL_NO: usize = 5usize;
const STATE_NO: usize = 12usize;
const MAX_ACTIONS: usize = 4usize;
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum TermKind {
    STOP = 0usize,
    Plus = 1usize,
    Mul = 2usize,
    LParen = 3usize,
    RParen = 4usize,
    Num = 5usize,
}
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
#[derive(Debug)]
pub enum Terminal {
    STOP,
    Plus,
    Mul,
    LParen,
    RParen,
    Num(calculator01_actions::Num),
}
#[derive(Debug)]
pub enum NonTerminal {
    E(calculator01_actions::E),
    T(calculator01_actions::T),
    F(calculator01_actions::F),
}
#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum ProdKind {
    EP1 = 1usize,
    EP2 = 2usize,
    TP1 = 3usize,
    TP2 = 4usize,
    FP1 = 5usize,
    FP2 = 6usize,
}
pub struct Calculator01ParserDefinition {
    actions: [[Action; TERMINAL_NO]; STATE_NO],
    gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO],
}
pub(crate) static PARSER_DEFINITION: Calculator01ParserDefinition = Calculator01ParserDefinition {
    actions: [
        [
            Error,
            Error,
            Error,
            Shift(StateIndex(1usize), TermIndex(3usize)),
            Error,
            Shift(StateIndex(2usize), TermIndex(5usize)),
        ],
        [
            Error,
            Error,
            Error,
            Shift(StateIndex(1usize), TermIndex(3usize)),
            Error,
            Shift(StateIndex(2usize), TermIndex(5usize)),
        ],
        [
            Reduce(ProdIndex(6usize), 1usize, NonTermIndex(4usize), "<?>"),
            Reduce(ProdIndex(6usize), 1usize, NonTermIndex(4usize), "<?>"),
            Reduce(ProdIndex(6usize), 1usize, NonTermIndex(4usize), "<?>"),
            Error,
            Reduce(ProdIndex(6usize), 1usize, NonTermIndex(4usize), "<?>"),
            Error,
        ],
        [
            Accept,
            Shift(StateIndex(7usize), TermIndex(1usize)),
            Error,
            Error,
            Error,
            Error,
        ],
        [
            Reduce(ProdIndex(2usize), 1usize, NonTermIndex(2usize), "<?>"),
            Reduce(ProdIndex(2usize), 1usize, NonTermIndex(2usize), "<?>"),
            Shift(StateIndex(8usize), TermIndex(2usize)),
            Error,
            Reduce(ProdIndex(2usize), 1usize, NonTermIndex(2usize), "<?>"),
            Error,
        ],
        [
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(3usize), "<?>"),
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(3usize), "<?>"),
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(3usize), "<?>"),
            Error,
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(3usize), "<?>"),
            Error,
        ],
        [
            Error,
            Shift(StateIndex(7usize), TermIndex(1usize)),
            Error,
            Error,
            Shift(StateIndex(9usize), TermIndex(4usize)),
            Error,
        ],
        [
            Error,
            Error,
            Error,
            Shift(StateIndex(1usize), TermIndex(3usize)),
            Error,
            Shift(StateIndex(2usize), TermIndex(5usize)),
        ],
        [
            Error,
            Error,
            Error,
            Shift(StateIndex(1usize), TermIndex(3usize)),
            Error,
            Shift(StateIndex(2usize), TermIndex(5usize)),
        ],
        [
            Reduce(ProdIndex(5usize), 3usize, NonTermIndex(4usize), "<?>"),
            Reduce(ProdIndex(5usize), 3usize, NonTermIndex(4usize), "<?>"),
            Reduce(ProdIndex(5usize), 3usize, NonTermIndex(4usize), "<?>"),
            Error,
            Reduce(ProdIndex(5usize), 3usize, NonTermIndex(4usize), "<?>"),
            Error,
        ],
        [
            Reduce(ProdIndex(1usize), 3usize, NonTermIndex(2usize), "<?>"),
            Reduce(ProdIndex(1usize), 3usize, NonTermIndex(2usize), "<?>"),
            Shift(StateIndex(8usize), TermIndex(2usize)),
            Error,
            Reduce(ProdIndex(1usize), 3usize, NonTermIndex(2usize), "<?>"),
            Error,
        ],
        [
            Reduce(ProdIndex(3usize), 3usize, NonTermIndex(3usize), "<?>"),
            Reduce(ProdIndex(3usize), 3usize, NonTermIndex(3usize), "<?>"),
            Reduce(ProdIndex(3usize), 3usize, NonTermIndex(3usize), "<?>"),
            Error,
            Reduce(ProdIndex(3usize), 3usize, NonTermIndex(3usize), "<?>"),
            Error,
        ],
    ],
    gotos: [
        [
            None,
            None,
            Some(StateIndex(3usize)),
            Some(StateIndex(4usize)),
            Some(StateIndex(5usize)),
        ],
        [
            None,
            None,
            Some(StateIndex(6usize)),
            Some(StateIndex(4usize)),
            Some(StateIndex(5usize)),
        ],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, Some(StateIndex(10usize)), Some(StateIndex(5usize))],
        [None, None, None, None, Some(StateIndex(11usize))],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
    ],
};
impl ParserDefinition for Calculator01ParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
    }
}
pub struct Calculator01Parser(LRParser<Calculator01ParserDefinition>);
impl<I, L, B> Parser<I, LRContext<I>, L, B> for Calculator01Parser
where
    I: Debug,
    L: Lexer<I, LRContext<I>>,
    B: LRBuilder<I>,
{
    fn parse(
        &mut self,
        context: LRContext<I>,
        lexer: L,
        builder: B,
    ) -> Result<B::Output> {
        Calculator01Parser::default().0.parse(context, lexer, builder)
    }
}
#[allow(dead_code)]
impl Calculator01Parser {
    pub fn parse_str<'i>(
        input: &'i str,
    ) -> Result<<Calculator01Builder as Builder>::Output> {
        let context = LRContext::new("<str>".to_string(), input);
        let lexer = LRStringLexer::new(&LEXER_DEFINITION);
        let builder = Calculator01Builder::new();
        Calculator01Parser::default().0.parse(context, lexer, builder)
    }
}
impl Default for Calculator01Parser {
    fn default() -> Self {
        Self(LRParser::new(&PARSER_DEFINITION))
    }
}
pub struct Calculator01LexerDefinition {
    terminals: TerminalInfos<TERMINAL_NO>,
    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO],
}
pub(crate) static LEXER_DEFINITION: Calculator01LexerDefinition = Calculator01LexerDefinition {
    terminals: [
        TerminalInfo {
            id: TermIndex(0usize),
            name: "STOP",
            location: None,
        },
        TerminalInfo {
            id: TermIndex(1usize),
            name: "Plus",
            location: None,
        },
        TerminalInfo {
            id: TermIndex(2usize),
            name: "Mul",
            location: None,
        },
        TerminalInfo {
            id: TermIndex(3usize),
            name: "LParen",
            location: None,
        },
        TerminalInfo {
            id: TermIndex(4usize),
            name: "RParen",
            location: None,
        },
        TerminalInfo {
            id: TermIndex(5usize),
            name: "Num",
            location: None,
        },
    ],
    terminals_for_state: [
        [Some(3usize), Some(5usize), None, None],
        [Some(3usize), Some(5usize), None, None],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
        [Some(0usize), Some(1usize), None, None],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
        [Some(1usize), Some(4usize), None, None],
        [Some(3usize), Some(5usize), None, None],
        [Some(3usize), Some(5usize), None, None],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
        [Some(0usize), Some(1usize), Some(2usize), Some(4usize)],
    ],
    recognizers: [
        |input: &str| {
            logn!("Recognizing <STOP> -- ");
            if input.len() == 0 {
                log!("recognized");
                Some("")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "Plus");
            if input.starts_with("+") {
                log!("recognized");
                Some("+")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "Mul");
            if input.starts_with("*") {
                log!("recognized");
                Some("*")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "LParen");
            if input.starts_with("(") {
                log!("recognized");
                Some("(")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "RParen");
            if input.starts_with(")") {
                log!("recognized");
                Some(")")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "Num");
            let regex = Regex::new(concat!("^", "\\d+")).unwrap();
            let match_str = regex.find(input);
            match match_str {
                Some(x) => {
                    let x_str = x.as_str();
                    log!("recognized <{}>", x_str);
                    Some(x_str)
                }
                None => {
                    log!("not recognized");
                    None
                }
            }
        },
    ],
};
impl LexerDefinition for Calculator01LexerDefinition {
    type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::Recognizer> {
        RecognizerIterator {
            terminals: &LEXER_DEFINITION.terminals,
            terminals_for_state: &LEXER_DEFINITION
                .terminals_for_state[state_index.0][..],
            recognizers: &LEXER_DEFINITION.recognizers,
            index: 0,
        }
    }
}
pub struct Calculator01Builder {
    res_stack: Vec<Symbol>,
}
impl Builder for Calculator01Builder {
    type Output = calculator01_actions::E;
    fn new() -> Self {
        Self { res_stack: vec![] }
    }
    fn get_result(&mut self) -> Result<Self::Output> {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::E(r)) => Ok(r),
            _ => panic!("Invalid result on the parsing stack!"),
        }
    }
}
impl<'i> LRBuilder<&'i str> for Calculator01Builder {
    fn shift_action(&mut self, term_idx: TermIndex, token: Token<&'i str>) {
        let termval = match TermKind::try_from(term_idx.0).unwrap() {
            TermKind::STOP => Terminal::STOP,
            TermKind::Plus => Terminal::Plus,
            TermKind::Mul => Terminal::Mul,
            TermKind::LParen => Terminal::LParen,
            TermKind::RParen => Terminal::RParen,
            TermKind::Num => Terminal::Num(calculator01_actions::num(token)),
        };
        self.res_stack.push(Symbol::Terminal(termval));
    }
    fn reduce_action(
        &mut self,
        prod_kind: ProdIndex,
        _prod_len: usize,
        _prod_str: &'static str,
    ) {
        let prod = match ProdKind::try_from(prod_kind.0).unwrap() {
            ProdKind::EP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::E(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::T(p1)),
                    ) => NonTerminal::E(calculator01_actions::e_v1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::EP2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::T(p0)) => {
                        NonTerminal::E(calculator01_actions::e_v2(p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::TP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::T(p0)),
                        _,
                        Symbol::NonTerminal(NonTerminal::F(p1)),
                    ) => NonTerminal::T(calculator01_actions::t_v1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::TP2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::F(p0)) => {
                        NonTerminal::T(calculator01_actions::t_v2(p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::E(p0)), _) => {
                        NonTerminal::F(calculator01_actions::f_v1(p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::FP2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::Terminal(Terminal::Num(p0)) => {
                        NonTerminal::F(calculator01_actions::f_v2(p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}