/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::Hash;
use rustemo::{
    Result, Input as InputT, Lexer, Token, TokenRecognizer as TokenRecognizerT, Parser,
    ParserDefinition, State as StateT, Builder,
};
use rustemo::regex::Regex;
use rustemo::once_cell::sync::Lazy;
use rustemo::StringLexer;
use rustemo::LRBuilder;
use super::calculator_actions;
use rustemo::{LRParser, LRContext};
use rustemo::Action::{self, Shift, Reduce, Accept};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use rustemo::colored::*;
pub type Input = str;
const STATE_COUNT: usize = 5usize;
const MAX_RECOGNIZERS: usize = 1usize;
#[allow(dead_code)]
const TERMINAL_COUNT: usize = 3usize;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Operand,
    Operator,
}
use TokenKind as TK;
impl From<TokenKind> for usize {
    fn from(t: TokenKind) -> Self {
        t as usize
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq)]
pub enum ProdKind {
    ExpressionP1,
}
use ProdKind as PK;
impl std::fmt::Debug for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::ExpressionP1 => "Expression: Operand Operator Operand",
        };
        write!(f, "{name}")
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum NonTermKind {
    EMPTY,
    AUG,
    Expression,
}
impl From<ProdKind> for NonTermKind {
    fn from(prod: ProdKind) -> Self {
        match prod {
            ProdKind::ExpressionP1 => NonTermKind::Expression,
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    #[default]
    AUGS0,
    OperandS1,
    ExpressionS2,
    OperatorS3,
    OperandS4,
}
impl StateT for State {
    fn default_layout() -> Option<Self> {
        None
    }
}
impl From<State> for usize {
    fn from(s: State) -> Self {
        s as usize
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            State::AUGS0 => "0:AUG",
            State::OperandS1 => "1:Operand",
            State::ExpressionS2 => "2:Expression",
            State::OperatorS3 => "3:Operator",
            State::OperandS4 => "4:Operand",
        };
        write!(f, "{name}")
    }
}
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Terminal {
    Operand(calculator_actions::Operand),
    Operator(calculator_actions::Operator),
}
#[derive(Debug)]
pub enum NonTerminal {
    Expression(calculator_actions::Expression),
}
type ActionFn = fn(token: TokenKind) -> Vec<Action<State, ProdKind>>;
pub struct CalculatorParserDefinition {
    actions: [ActionFn; STATE_COUNT],
    gotos: [fn(nonterm: NonTermKind) -> State; STATE_COUNT],
    token_kinds: [[Option<(TokenKind, bool)>; MAX_RECOGNIZERS]; STATE_COUNT],
}
fn action_aug_s0(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Operand => Vec::from(&[Shift(State::OperandS1)]),
        _ => vec![],
    }
}
fn action_operand_s1(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Operator => Vec::from(&[Shift(State::OperatorS3)]),
        _ => vec![],
    }
}
fn action_expression_s2(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Accept]),
        _ => vec![],
    }
}
fn action_operator_s3(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::Operand => Vec::from(&[Shift(State::OperandS4)]),
        _ => vec![],
    }
}
fn action_operand_s4(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
    match token_kind {
        TK::STOP => Vec::from(&[Reduce(PK::ExpressionP1, 3usize)]),
        _ => vec![],
    }
}
fn goto_aug_s0(nonterm_kind: NonTermKind) -> State {
    match nonterm_kind {
        NonTermKind::Expression => State::ExpressionS2,
        _ => {
            panic!(
                "Invalid terminal kind ({nonterm_kind:?}) for GOTO state ({:?}).",
                State::AUGS0
            )
        }
    }
}
fn goto_invalid(_nonterm_kind: NonTermKind) -> State {
    panic!("Invalid GOTO entry!");
}
pub(crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
        action_aug_s0,
        action_operand_s1,
        action_expression_s2,
        action_operator_s3,
        action_operand_s4,
    ],
    gotos: [goto_aug_s0, goto_invalid, goto_invalid, goto_invalid, goto_invalid],
    token_kinds: [
        [Some((TK::Operand, false))],
        [Some((TK::Operator, false))],
        [Some((TK::STOP, false))],
        [Some((TK::Operand, false))],
        [Some((TK::STOP, false))],
    ],
};
impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind>
for CalculatorParserDefinition {
    fn actions(&self, state: State, token: TokenKind) -> Vec<Action<State, ProdKind>> {
        PARSER_DEFINITION.actions[state as usize](token)
    }
    fn goto(&self, state: State, nonterm: NonTermKind) -> State {
        PARSER_DEFINITION.gotos[state as usize](nonterm)
    }
    fn expected_token_kinds(&self, state: State) -> Vec<(TokenKind, bool)> {
        PARSER_DEFINITION.token_kinds[state as usize].iter().map_while(|t| *t).collect()
    }
    fn longest_match() -> bool {
        true
    }
    fn grammar_order() -> bool {
        true
    }
}
pub(crate) type Context<'i, I> = LRContext<'i, I, State, TokenKind>;
pub struct CalculatorParser<
    'i,
    I: InputT + ?Sized,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B,
>(
    LRParser<
        'i,
        Context<'i, I>,
        State,
        ProdKind,
        TokenKind,
        NonTermKind,
        CalculatorParserDefinition,
        L,
        B,
        I,
    >,
);
#[allow(dead_code)]
impl<
    'i,
> CalculatorParser<
    'i,
    Input,
    StringLexer<Context<'i, Input>, State, TokenKind, TokenRecognizer, TERMINAL_COUNT>,
    DefaultBuilder,
> {
    pub fn new() -> Self {
        Self(
            LRParser::new(
                &PARSER_DEFINITION,
                State::default(),
                false,
                false,
                StringLexer::new(true, &RECOGNIZERS),
                DefaultBuilder::new(),
            ),
        )
    }
}
#[allow(dead_code)]
impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, State, TokenKind>
for CalculatorParser<'i, I, L, B>
where
    I: InputT + ?Sized + Debug,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B: LRBuilder<'i, I, Context<'i, I>, State, ProdKind, TokenKind>,
{
    type Output = B::Output;
    fn parse(&self, input: &'i I) -> Result<Self::Output> {
        self.0.parse(input)
    }
    fn parse_with_context(
        &self,
        context: &mut Context<'i, I>,
        input: &'i I,
    ) -> Result<Self::Output> {
        self.0.parse_with_context(context, input)
    }
    fn parse_file<'a, F: AsRef<std::path::Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.0.parse_file(file)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(Lazy<Regex>),
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenRecognizer(TokenKind, Recognizer);
impl<'i> TokenRecognizerT<'i> for TokenRecognizer {
    fn recognize(&self, input: &'i str) -> Option<&'i str> {
        match &self {
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::StrMatch(s)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                if input.starts_with(s) {
                    log!("{}", "recognized".bold().green());
                    Some(s)
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::RegexMatch(r)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                let match_str = r.find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("{} '{}'", "recognized".bold().green(), x_str);
                        Some(x_str)
                    }
                    _ => {
                        log!("{}", "not recognized".red());
                        None
                    }
                }
            }
            TokenRecognizer(_, Recognizer::Stop) => {
                logn!("{} STOP -- ", "    Recognizing".green());
                if input.is_empty() {
                    log!("{}", "recognized".bold().green());
                    Some("")
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
        }
    }
}
pub(crate) static RECOGNIZERS: [TokenRecognizer; TERMINAL_COUNT] = [
    TokenRecognizer(TokenKind::STOP, Recognizer::Stop),
    TokenRecognizer(
        TokenKind::Operand,
        Recognizer::RegexMatch(
            Lazy::new(|| { Regex::new(concat!("^", "\\d+(\\.\\d+)?")).unwrap() }),
        ),
    ),
    TokenRecognizer(
        TokenKind::Operator,
        Recognizer::RegexMatch(
            Lazy::new(|| { Regex::new(concat!("^", "\\+|-|\\*|/")).unwrap() }),
        ),
    ),
];
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl DefaultBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { res_stack: vec![] }
    }
}
impl Builder for DefaultBuilder {
    type Output = calculator_actions::Expression;
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::Expression(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, Context<'i, Input>, State, ProdKind, TokenKind>
for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &Context<'i, Input>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Operand => {
                Terminal::Operand(calculator_actions::operand(context, token))
            }
            TokenKind::Operator => {
                Terminal::Operator(calculator_actions::operator(context, token))
            }
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &Context<'i, Input>,
        prod: ProdKind,
        prod_len: usize,
    ) {
        let prod = match prod {
            ProdKind::ExpressionP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::Operand(p0)),
                        Symbol::Terminal(Terminal::Operator(p1)),
                        Symbol::Terminal(Terminal::Operand(p2)),
                    ) => {
                        NonTerminal::Expression(
                            calculator_actions::expression_c1(context, p0, p1, p2),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
