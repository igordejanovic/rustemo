// Generated on 2022-07-29 17:00:07.835767 from bootstrap.py. Do not edit!

use regex::Regex;
use std::convert::TryFrom;
use std::path::Path;
use std::fmt::Debug;

use rustemo_rt::lexer::{Lexer, Token};
use rustemo_rt::parser::Parser;
use rustemo_rt::error::RustemoResult;
use rustemo_rt::lr::lexer::{LRStringLexer, LRContext, LexerDefinition, RecognizerIterator};
use rustemo_rt::lr::builder::LRBuilder;
use rustemo_rt::lr::parser::{LRParser, ParserDefinition};
use rustemo_rt::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
use rustemo_rt::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
use rustemo_rt::builder::Builder;
use rustemo_rt::grammar::{TerminalInfo, TerminalInfos, TerminalsState};
use rustemo_rt::debug::{log, logn};
use super::rustemo_types::{TermKind, ProdKind, Terminal, NonTerminal, Symbol};

use super::rustemo_actions::*;

const TERMINAL_NO: usize = 44;
const NONTERMINAL_NO: usize = 36;
const STATE_NO: usize = 128;
const MAX_ACTIONS: usize = 15;

pub struct RustemoParserDefinition {
    actions: [[Action; TERMINAL_NO]; STATE_NO],
    gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
}

pub(in crate) static PARSER_DEFINITION: RustemoParserDefinition = RustemoParserDefinition {
   actions: [
   // State 0:S'
    [Shift(StateIndex(4), TermIndex(0)), Shift(StateIndex(6), TermIndex(1)), Error, Error, Error, Shift(StateIndex(10), TermIndex(5)), Shift(StateIndex(8), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 1:PGFile
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Accept],
   // State 2:ProductionRules
    [Shift(StateIndex(11), TermIndex(0)), Error, Error, Error, Error, Shift(StateIndex(10), TermIndex(5)), Shift(StateIndex(8), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(1), 1, NonTermIndex(1), "1: PGFile = ProductionRules")],
   // State 3:Imports
    [Error, Shift(StateIndex(6), TermIndex(1)), Error, Error, Error, Shift(StateIndex(10), TermIndex(5)), Shift(StateIndex(8), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 4:terminals
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 5:Import
    [Error, Reduce(ProdIndex(7), 1, NonTermIndex(2), "7: Imports = Import"), Error, Error, Error, Reduce(ProdIndex(7), 1, NonTermIndex(2), "7: Imports = Import"), Reduce(ProdIndex(7), 1, NonTermIndex(2), "7: Imports = Import"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 6:import
    [Error, Error, Shift(StateIndex(20), TermIndex(2)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 7:ProductionRuleWithAction
    [Reduce(ProdIndex(11), 1, NonTermIndex(4), "11: ProductionRules = ProductionRuleWithAction"), Error, Error, Error, Error, Reduce(ProdIndex(11), 1, NonTermIndex(4), "11: ProductionRules = ProductionRuleWithAction"), Reduce(ProdIndex(11), 1, NonTermIndex(4), "11: ProductionRules = ProductionRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(11), 1, NonTermIndex(4), "11: ProductionRules = ProductionRuleWithAction")],
   // State 8:Action
    [Error, Error, Error, Error, Error, Shift(StateIndex(10), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 9:ProductionRule
    [Reduce(ProdIndex(13), 1, NonTermIndex(5), "13: ProductionRuleWithAction = ProductionRule"), Error, Error, Error, Error, Reduce(ProdIndex(13), 1, NonTermIndex(5), "13: ProductionRuleWithAction = ProductionRule"), Reduce(ProdIndex(13), 1, NonTermIndex(5), "13: ProductionRuleWithAction = ProductionRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(13), 1, NonTermIndex(5), "13: ProductionRuleWithAction = ProductionRule")],
   // State 10:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(22), TermIndex(7)), Shift(StateIndex(23), TermIndex(8)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 11:terminals
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 12:ProductionRuleWithAction
    [Reduce(ProdIndex(10), 2, NonTermIndex(4), "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Error, Error, Error, Error, Reduce(ProdIndex(10), 2, NonTermIndex(4), "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Reduce(ProdIndex(10), 2, NonTermIndex(4), "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(10), 2, NonTermIndex(4), "10: ProductionRules = ProductionRules ProductionRuleWithAction")],
   // State 13:ProductionRules
    [Shift(StateIndex(25), TermIndex(0)), Error, Error, Error, Error, Shift(StateIndex(10), TermIndex(5)), Shift(StateIndex(8), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(2), 2, NonTermIndex(1), "2: PGFile = Imports ProductionRules")],
   // State 14:Import
    [Error, Reduce(ProdIndex(6), 2, NonTermIndex(2), "6: Imports = Imports Import"), Error, Error, Error, Reduce(ProdIndex(6), 2, NonTermIndex(2), "6: Imports = Imports Import"), Reduce(ProdIndex(6), 2, NonTermIndex(2), "6: Imports = Imports Import"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 15:TerminalRules
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(5), 2, NonTermIndex(1), "5: PGFile = terminals TerminalRules")],
   // State 16:TerminalRuleWithAction
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(21), 1, NonTermIndex(9), "21: TerminalRules = TerminalRuleWithAction"), Reduce(ProdIndex(21), 1, NonTermIndex(9), "21: TerminalRules = TerminalRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(21), 1, NonTermIndex(9), "21: TerminalRules = TerminalRuleWithAction")],
   // State 17:Action
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 18:TerminalRule
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(23), 1, NonTermIndex(10), "23: TerminalRuleWithAction = TerminalRule"), Reduce(ProdIndex(23), 1, NonTermIndex(10), "23: TerminalRuleWithAction = TerminalRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(23), 1, NonTermIndex(10), "23: TerminalRuleWithAction = TerminalRule")],
   // State 19:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(28), TermIndex(7)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 20:StrConst
    [Error, Error, Error, Shift(StateIndex(29), TermIndex(3)), Shift(StateIndex(30), TermIndex(4)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 21:ProductionRule
    [Reduce(ProdIndex(12), 2, NonTermIndex(5), "12: ProductionRuleWithAction = Action ProductionRule"), Error, Error, Error, Error, Reduce(ProdIndex(12), 2, NonTermIndex(5), "12: ProductionRuleWithAction = Action ProductionRule"), Reduce(ProdIndex(12), 2, NonTermIndex(5), "12: ProductionRuleWithAction = Action ProductionRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(12), 2, NonTermIndex(5), "12: ProductionRuleWithAction = Action ProductionRule")],
   // State 22::
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(41), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 23:{
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Shift(StateIndex(45), TermIndex(11)), Shift(StateIndex(46), TermIndex(12)), Shift(StateIndex(47), TermIndex(13)), Shift(StateIndex(48), TermIndex(14)), Shift(StateIndex(49), TermIndex(15)), Shift(StateIndex(50), TermIndex(16)), Shift(StateIndex(51), TermIndex(17)), Shift(StateIndex(52), TermIndex(18)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 24:TerminalRules
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(3), 3, NonTermIndex(1), "3: PGFile = ProductionRules terminals TerminalRules")],
   // State 25:terminals
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 26:TerminalRuleWithAction
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(20), 2, NonTermIndex(9), "20: TerminalRules = TerminalRules TerminalRuleWithAction"), Reduce(ProdIndex(20), 2, NonTermIndex(9), "20: TerminalRules = TerminalRules TerminalRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(20), 2, NonTermIndex(9), "20: TerminalRules = TerminalRules TerminalRuleWithAction")],
   // State 27:TerminalRule
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(22), 2, NonTermIndex(10), "22: TerminalRuleWithAction = Action TerminalRule"), Reduce(ProdIndex(22), 2, NonTermIndex(10), "22: TerminalRuleWithAction = Action TerminalRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(22), 2, NonTermIndex(10), "22: TerminalRuleWithAction = Action TerminalRule")],
   // State 28::
    [Error, Error, Shift(StateIndex(59), TermIndex(2)), Shift(StateIndex(57), TermIndex(3)), Error, Error, Error, Error, Shift(StateIndex(58), TermIndex(8)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(60), TermIndex(37)), Error, Error, Error, Error, Error, Error],
   // State 29:;
    [Error, Reduce(ProdIndex(8), 3, NonTermIndex(3), "8: Import = import StrConst ;"), Error, Error, Error, Reduce(ProdIndex(8), 3, NonTermIndex(3), "8: Import = import StrConst ;"), Reduce(ProdIndex(8), 3, NonTermIndex(3), "8: Import = import StrConst ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 30:as
    [Error, Error, Error, Error, Error, Shift(StateIndex(61), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 31:ProductionRuleRHS
    [Error, Error, Error, Shift(StateIndex(62), TermIndex(3)), Error, Error, Error, Error, Error, Error, Shift(StateIndex(63), TermIndex(10)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 32:Production
    [Error, Error, Error, Reduce(ProdIndex(17), 1, NonTermIndex(7), "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(17), 1, NonTermIndex(7), "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(17), 1, NonTermIndex(7), "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 33:Assignments
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Reduce(ProdIndex(18), 1, NonTermIndex(8), "18: Production = Assignments"), Error, Shift(StateIndex(41), TermIndex(5)), Error, Error, Shift(StateIndex(64), TermIndex(8)), Error, Reduce(ProdIndex(18), 1, NonTermIndex(8), "18: Production = Assignments"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Reduce(ProdIndex(18), 1, NonTermIndex(8), "18: Production = Assignments"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 34:Assignment
    [Error, Error, Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Error, Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Error, Error, Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Error, Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Reduce(ProdIndex(56), 1, NonTermIndex(19), "56: Assignments = Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 35:PlainAssignment
    [Error, Error, Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Error, Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Error, Error, Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Error, Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Reduce(ProdIndex(52), 1, NonTermIndex(18), "52: Assignment = PlainAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 36:BoolAssignment
    [Error, Error, Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Error, Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Error, Error, Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Error, Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Reduce(ProdIndex(53), 1, NonTermIndex(18), "53: Assignment = BoolAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 37:GrammarSymbolReference
    [Error, Error, Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Error, Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Error, Error, Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Error, Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Reduce(ProdIndex(54), 1, NonTermIndex(18), "54: Assignment = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 38:GrammarSymbol
    [Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Shift(StateIndex(68), TermIndex(29)), Shift(StateIndex(69), TermIndex(30)), Shift(StateIndex(70), TermIndex(31)), Shift(StateIndex(71), TermIndex(32)), Shift(StateIndex(72), TermIndex(33)), Shift(StateIndex(73), TermIndex(34)), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 39:ProductionGroup
    [Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Reduce(ProdIndex(63), 0, NonTermIndex(24), "63: OptRepeatOperator = EMPTY"), Shift(StateIndex(68), TermIndex(29)), Shift(StateIndex(69), TermIndex(30)), Shift(StateIndex(70), TermIndex(31)), Shift(StateIndex(71), TermIndex(32)), Shift(StateIndex(72), TermIndex(33)), Shift(StateIndex(73), TermIndex(34)), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 40:(
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(41), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 41:Name
    [Error, Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(77), TermIndex(25)), Shift(StateIndex(76), TermIndex(26)), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 42:StrConst
    [Error, Error, Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Error, Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Error, Error, Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Error, Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Reduce(ProdIndex(76), 1, NonTermIndex(29), "76: GrammarSymbol = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 43:ProductionMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(78), TermIndex(9)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(79), TermIndex(19)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 44:ProductionMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(38), 1, NonTermIndex(13), "38: ProductionMetaDatas = ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(38), 1, NonTermIndex(13), "38: ProductionMetaDatas = ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 45:left
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(28), 1, NonTermIndex(12), "28: ProductionMetaData = left"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(28), 1, NonTermIndex(12), "28: ProductionMetaData = left"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 46:reduce
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(29), 1, NonTermIndex(12), "29: ProductionMetaData = reduce"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(29), 1, NonTermIndex(12), "29: ProductionMetaData = reduce"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 47:right
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(30), 1, NonTermIndex(12), "30: ProductionMetaData = right"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(30), 1, NonTermIndex(12), "30: ProductionMetaData = right"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 48:shift
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(31), 1, NonTermIndex(12), "31: ProductionMetaData = shift"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(31), 1, NonTermIndex(12), "31: ProductionMetaData = shift"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 49:dynamic
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(32), 1, NonTermIndex(12), "32: ProductionMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(32), 1, NonTermIndex(12), "32: ProductionMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 50:nops
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(33), 1, NonTermIndex(12), "33: ProductionMetaData = nops"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(33), 1, NonTermIndex(12), "33: ProductionMetaData = nops"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 51:nopse
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(34), 1, NonTermIndex(12), "34: ProductionMetaData = nopse"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(34), 1, NonTermIndex(12), "34: ProductionMetaData = nopse"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 52:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(35), 1, NonTermIndex(12), "35: ProductionMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(35), 1, NonTermIndex(12), "35: ProductionMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 53:UserMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(36), 1, NonTermIndex(12), "36: ProductionMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(36), 1, NonTermIndex(12), "36: ProductionMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 54:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(80), TermIndex(7)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 55:TerminalRules
    [Error, Error, Error, Error, Error, Shift(StateIndex(19), TermIndex(5)), Shift(StateIndex(17), TermIndex(6)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(4), 4, NonTermIndex(1), "4: PGFile = Imports ProductionRules terminals TerminalRules")],
   // State 56:Recognizer
    [Error, Error, Error, Shift(StateIndex(81), TermIndex(3)), Error, Error, Error, Error, Shift(StateIndex(82), TermIndex(8)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 57:;
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(25), 3, NonTermIndex(11), "25: TerminalRule = Name : ;"), Reduce(ProdIndex(25), 3, NonTermIndex(11), "25: TerminalRule = Name : ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(25), 3, NonTermIndex(11), "25: TerminalRule = Name : ;")],
   // State 58:{
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(88), TermIndex(15)), Error, Error, Shift(StateIndex(89), TermIndex(18)), Error, Shift(StateIndex(85), TermIndex(20)), Shift(StateIndex(86), TermIndex(21)), Shift(StateIndex(87), TermIndex(22)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 59:StrConst
    [Error, Error, Error, Reduce(ProdIndex(77), 1, NonTermIndex(30), "77: Recognizer = StrConst"), Error, Error, Error, Error, Reduce(ProdIndex(77), 1, NonTermIndex(30), "77: Recognizer = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 60:RegExTerm
    [Error, Error, Error, Reduce(ProdIndex(78), 1, NonTermIndex(30), "78: Recognizer = RegExTerm"), Error, Error, Error, Error, Reduce(ProdIndex(78), 1, NonTermIndex(30), "78: Recognizer = RegExTerm"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 61:Name
    [Error, Error, Error, Shift(StateIndex(91), TermIndex(3)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 62:;
    [Reduce(ProdIndex(14), 4, NonTermIndex(6), "14: ProductionRule = Name : ProductionRuleRHS ;"), Error, Error, Error, Error, Reduce(ProdIndex(14), 4, NonTermIndex(6), "14: ProductionRule = Name : ProductionRuleRHS ;"), Reduce(ProdIndex(14), 4, NonTermIndex(6), "14: ProductionRule = Name : ProductionRuleRHS ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(14), 4, NonTermIndex(6), "14: ProductionRule = Name : ProductionRuleRHS ;")],
   // State 63:|
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(41), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 64:{
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Shift(StateIndex(45), TermIndex(11)), Shift(StateIndex(46), TermIndex(12)), Shift(StateIndex(47), TermIndex(13)), Shift(StateIndex(48), TermIndex(14)), Shift(StateIndex(49), TermIndex(15)), Shift(StateIndex(50), TermIndex(16)), Shift(StateIndex(51), TermIndex(17)), Shift(StateIndex(52), TermIndex(18)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 65:Assignment
    [Error, Error, Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Error, Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Error, Error, Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Error, Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Reduce(ProdIndex(55), 2, NonTermIndex(19), "55: Assignments = Assignments Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 66:OptRepeatOperator
    [Error, Error, Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Reduce(ProdIndex(60), 2, NonTermIndex(23), "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 67:RepeatOperator
    [Error, Error, Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Error, Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Error, Error, Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Error, Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Reduce(ProdIndex(62), 1, NonTermIndex(24), "62: OptRepeatOperator = RepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 68:*
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 69:*!
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 70:+
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 71:+!
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 72:?
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 73:?!
    [Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(ProdIndex(71), 0, NonTermIndex(26), "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(StateIndex(95), TermIndex(35)), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 74:OptRepeatOperator
    [Error, Error, Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Reduce(ProdIndex(61), 2, NonTermIndex(23), "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 75:ProductionRuleRHS
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(63), TermIndex(10)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(101), TermIndex(28)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 76:?=
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(103), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 77:=
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(103), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 78:}
    [Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(105), TermIndex(7)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 79:,
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Shift(StateIndex(45), TermIndex(11)), Shift(StateIndex(46), TermIndex(12)), Shift(StateIndex(47), TermIndex(13)), Shift(StateIndex(48), TermIndex(14)), Shift(StateIndex(49), TermIndex(15)), Shift(StateIndex(50), TermIndex(16)), Shift(StateIndex(51), TermIndex(17)), Shift(StateIndex(52), TermIndex(18)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 80::
    [Error, Error, Shift(StateIndex(111), TermIndex(2)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(108), TermIndex(18)), Error, Error, Error, Error, Shift(StateIndex(109), TermIndex(23)), Shift(StateIndex(110), TermIndex(24)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 81:;
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(24), 4, NonTermIndex(11), "24: TerminalRule = Name : Recognizer ;"), Reduce(ProdIndex(24), 4, NonTermIndex(11), "24: TerminalRule = Name : Recognizer ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(24), 4, NonTermIndex(11), "24: TerminalRule = Name : Recognizer ;")],
   // State 82:{
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(88), TermIndex(15)), Error, Error, Shift(StateIndex(89), TermIndex(18)), Error, Shift(StateIndex(85), TermIndex(20)), Shift(StateIndex(86), TermIndex(21)), Shift(StateIndex(87), TermIndex(22)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 83:TerminalMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(113), TermIndex(9)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(114), TermIndex(19)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 84:TerminalMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(46), 1, NonTermIndex(15), "46: TerminalMetaDatas = TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(46), 1, NonTermIndex(15), "46: TerminalMetaDatas = TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 85:prefer
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(39), 1, NonTermIndex(14), "39: TerminalMetaData = prefer"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(39), 1, NonTermIndex(14), "39: TerminalMetaData = prefer"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 86:finish
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(40), 1, NonTermIndex(14), "40: TerminalMetaData = finish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(40), 1, NonTermIndex(14), "40: TerminalMetaData = finish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 87:nofinish
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(41), 1, NonTermIndex(14), "41: TerminalMetaData = nofinish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(41), 1, NonTermIndex(14), "41: TerminalMetaData = nofinish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 88:dynamic
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(42), 1, NonTermIndex(14), "42: TerminalMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(42), 1, NonTermIndex(14), "42: TerminalMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 89:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(43), 1, NonTermIndex(14), "43: TerminalMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(43), 1, NonTermIndex(14), "43: TerminalMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 90:UserMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(44), 1, NonTermIndex(14), "44: TerminalMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(44), 1, NonTermIndex(14), "44: TerminalMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 91:;
    [Error, Reduce(ProdIndex(9), 5, NonTermIndex(3), "9: Import = import StrConst as Name ;"), Error, Error, Error, Reduce(ProdIndex(9), 5, NonTermIndex(3), "9: Import = import StrConst as Name ;"), Reduce(ProdIndex(9), 5, NonTermIndex(3), "9: Import = import StrConst as Name ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 92:Production
    [Error, Error, Error, Reduce(ProdIndex(16), 3, NonTermIndex(7), "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(16), 3, NonTermIndex(7), "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(16), 3, NonTermIndex(7), "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 93:ProductionMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(115), TermIndex(9)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(79), TermIndex(19)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 94:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Reduce(ProdIndex(64), 2, NonTermIndex(25), "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 95:[
    [Error, Error, Error, Error, Error, Shift(StateIndex(118), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 96:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(65), 2, NonTermIndex(25), "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 97:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Reduce(ProdIndex(66), 2, NonTermIndex(25), "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 98:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(67), 2, NonTermIndex(25), "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 99:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Reduce(ProdIndex(68), 2, NonTermIndex(25), "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 100:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Reduce(ProdIndex(69), 2, NonTermIndex(25), "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 101:)
    [Error, Error, Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(ProdIndex(59), 3, NonTermIndex(22), "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 102:GrammarSymbolReference
    [Error, Error, Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Reduce(ProdIndex(58), 3, NonTermIndex(21), "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 103:Name
    [Error, Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Reduce(ProdIndex(75), 1, NonTermIndex(29), "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 104:GrammarSymbolReference
    [Error, Error, Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Reduce(ProdIndex(57), 3, NonTermIndex(20), "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 105::
    [Error, Error, Shift(StateIndex(42), TermIndex(2)), Error, Error, Shift(StateIndex(41), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(40), TermIndex(27)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 106:ProductionMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(37), 3, NonTermIndex(13), "37: ProductionMetaDatas = ProductionMetaDatas , ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(37), 3, NonTermIndex(13), "37: ProductionMetaDatas = ProductionMetaDatas , ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 107:Const
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(47), 3, NonTermIndex(16), "47: UserMetaData = Name : Const"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(47), 3, NonTermIndex(16), "47: UserMetaData = Name : Const"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 108:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(48), 1, NonTermIndex(17), "48: Const = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(48), 1, NonTermIndex(17), "48: Const = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 109:FloatConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(49), 1, NonTermIndex(17), "49: Const = FloatConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(49), 1, NonTermIndex(17), "49: Const = FloatConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 110:BoolConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(50), 1, NonTermIndex(17), "50: Const = BoolConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(50), 1, NonTermIndex(17), "50: Const = BoolConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 111:StrConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(51), 1, NonTermIndex(17), "51: Const = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(51), 1, NonTermIndex(17), "51: Const = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 112:TerminalMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(120), TermIndex(9)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(114), TermIndex(19)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 113:}
    [Error, Error, Error, Shift(StateIndex(121), TermIndex(3)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 114:,
    [Error, Error, Error, Error, Error, Shift(StateIndex(54), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(88), TermIndex(15)), Error, Error, Shift(StateIndex(89), TermIndex(18)), Error, Shift(StateIndex(85), TermIndex(20)), Shift(StateIndex(86), TermIndex(21)), Shift(StateIndex(87), TermIndex(22)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 115:}
    [Error, Error, Error, Reduce(ProdIndex(19), 4, NonTermIndex(8), "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(19), 4, NonTermIndex(8), "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(19), 4, NonTermIndex(8), "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 116:OptionalRepeatModifiers
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(124), TermIndex(19)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(StateIndex(123), TermIndex(36)), Error, Error, Error, Error, Error, Error, Error],
   // State 117:OptionalRepeatModifier
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(73), 1, NonTermIndex(27), "73: OptionalRepeatModifiers = OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(73), 1, NonTermIndex(27), "73: OptionalRepeatModifiers = OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error],
   // State 118:Name
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(74), 1, NonTermIndex(28), "74: OptionalRepeatModifier = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(74), 1, NonTermIndex(28), "74: OptionalRepeatModifier = Name"), Error, Error, Error, Error, Error, Error, Error],
   // State 119:ProductionRuleRHS
    [Error, Error, Error, Shift(StateIndex(125), TermIndex(3)), Error, Error, Error, Error, Error, Error, Shift(StateIndex(63), TermIndex(10)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 120:}
    [Error, Error, Error, Shift(StateIndex(126), TermIndex(3)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 121:;
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(27), 6, NonTermIndex(11), "27: TerminalRule = Name : { TerminalMetaDatas } ;"), Reduce(ProdIndex(27), 6, NonTermIndex(11), "27: TerminalRule = Name : { TerminalMetaDatas } ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(27), 6, NonTermIndex(11), "27: TerminalRule = Name : { TerminalMetaDatas } ;")],
   // State 122:TerminalMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(45), 3, NonTermIndex(15), "45: TerminalMetaDatas = TerminalMetaDatas , TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(45), 3, NonTermIndex(15), "45: TerminalMetaDatas = TerminalMetaDatas , TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 123:]
    [Error, Error, Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Reduce(ProdIndex(70), 3, NonTermIndex(26), "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 124:,
    [Error, Error, Error, Error, Error, Shift(StateIndex(118), TermIndex(5)), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 125:;
    [Reduce(ProdIndex(15), 7, NonTermIndex(6), "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Error, Error, Error, Error, Reduce(ProdIndex(15), 7, NonTermIndex(6), "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Reduce(ProdIndex(15), 7, NonTermIndex(6), "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(15), 7, NonTermIndex(6), "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;")],
   // State 126:;
    [Error, Error, Error, Error, Error, Reduce(ProdIndex(26), 7, NonTermIndex(11), "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;"), Reduce(ProdIndex(26), 7, NonTermIndex(11), "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(26), 7, NonTermIndex(11), "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;")],
   // State 127:OptionalRepeatModifier
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(72), 3, NonTermIndex(27), "72: OptionalRepeatModifiers = OptionalRepeatModifiers , OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(ProdIndex(72), 3, NonTermIndex(27), "72: OptionalRepeatModifiers = OptionalRepeatModifiers , OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error]   
],

   gotos: [
   // State 0:S'
    [None, Some(StateIndex(1)), Some(StateIndex(3)), Some(StateIndex(5)), Some(StateIndex(2)), Some(StateIndex(7)), Some(StateIndex(9)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 1:PGFile
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 2:ProductionRules
    [None, None, None, None, None, Some(StateIndex(12)), Some(StateIndex(9)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 3:Imports
    [None, None, None, Some(StateIndex(14)), Some(StateIndex(13)), Some(StateIndex(7)), Some(StateIndex(9)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 4:terminals
    [None, None, None, None, None, None, None, None, None, Some(StateIndex(15)), Some(StateIndex(16)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 5:Import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 6:import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 7:ProductionRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 8:Action
    [None, None, None, None, None, None, Some(StateIndex(21)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 9:ProductionRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 10:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 11:terminals
    [None, None, None, None, None, None, None, None, None, Some(StateIndex(24)), Some(StateIndex(16)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 12:ProductionRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 13:ProductionRules
    [None, None, None, None, None, Some(StateIndex(12)), Some(StateIndex(9)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 14:Import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 15:TerminalRules
    [None, None, None, None, None, None, None, None, None, None, Some(StateIndex(26)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 16:TerminalRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 17:Action
    [None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(27)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 18:TerminalRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 19:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 20:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 21:ProductionRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 22::
    [None, None, None, None, None, None, None, Some(StateIndex(31)), Some(StateIndex(32)), None, None, None, None, None, None, None, None, None, Some(StateIndex(34)), Some(StateIndex(33)), Some(StateIndex(35)), Some(StateIndex(36)), Some(StateIndex(39)), Some(StateIndex(37)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 23:{
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(44)), Some(StateIndex(43)), None, None, Some(StateIndex(53)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 24:TerminalRules
    [None, None, None, None, None, None, None, None, None, None, Some(StateIndex(26)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 25:terminals
    [None, None, None, None, None, None, None, None, None, Some(StateIndex(55)), Some(StateIndex(16)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 26:TerminalRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 27:TerminalRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 28::
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(56)), None, None, None, None, None],
   // State 29:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 30:as
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 31:ProductionRuleRHS
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 32:Production
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 33:Assignments
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(65)), None, Some(StateIndex(35)), Some(StateIndex(36)), Some(StateIndex(39)), Some(StateIndex(37)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 34:Assignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 35:PlainAssignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 36:BoolAssignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 37:GrammarSymbolReference
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 38:GrammarSymbol
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(66)), Some(StateIndex(67)), None, None, None, None, None, None, None, None, None, None],
   // State 39:ProductionGroup
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(74)), Some(StateIndex(67)), None, None, None, None, None, None, None, None, None, None],
   // State 40:(
    [None, None, None, None, None, None, None, Some(StateIndex(75)), Some(StateIndex(32)), None, None, None, None, None, None, None, None, None, Some(StateIndex(34)), Some(StateIndex(33)), Some(StateIndex(35)), Some(StateIndex(36)), Some(StateIndex(39)), Some(StateIndex(37)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 41:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 42:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 43:ProductionMetaDatas
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 44:ProductionMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 45:left
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 46:reduce
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 47:right
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 48:shift
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 49:dynamic
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 50:nops
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 51:nopse
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 52:IntConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 53:UserMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 54:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 55:TerminalRules
    [None, None, None, None, None, None, None, None, None, None, Some(StateIndex(26)), Some(StateIndex(18)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 56:Recognizer
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 57:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 58:{
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(84)), Some(StateIndex(83)), Some(StateIndex(90)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 59:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 60:RegExTerm
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 61:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 62:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 63:|
    [None, None, None, None, None, None, None, None, Some(StateIndex(92)), None, None, None, None, None, None, None, None, None, Some(StateIndex(34)), Some(StateIndex(33)), Some(StateIndex(35)), Some(StateIndex(36)), Some(StateIndex(39)), Some(StateIndex(37)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 64:{
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(44)), Some(StateIndex(93)), None, None, Some(StateIndex(53)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 65:Assignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 66:OptRepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 67:RepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 68:*
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(94)), None, None, None, None, None, None, None, None, None],
   // State 69:*!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(96)), None, None, None, None, None, None, None, None, None],
   // State 70:+
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(97)), None, None, None, None, None, None, None, None, None],
   // State 71:+!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(98)), None, None, None, None, None, None, None, None, None],
   // State 72:?
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(99)), None, None, None, None, None, None, None, None, None],
   // State 73:?!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(100)), None, None, None, None, None, None, None, None, None],
   // State 74:OptRepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 75:ProductionRuleRHS
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 76:?=
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(39)), Some(StateIndex(102)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 77:=
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(39)), Some(StateIndex(104)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 78:}
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 79:,
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(106)), None, None, None, Some(StateIndex(53)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 80::
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(107)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 81:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 82:{
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(84)), Some(StateIndex(112)), Some(StateIndex(90)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 83:TerminalMetaDatas
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 84:TerminalMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 85:prefer
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 86:finish
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 87:nofinish
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 88:dynamic
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 89:IntConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 90:UserMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 91:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 92:Production
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 93:ProductionMetaDatas
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 94:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 95:[
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(116)), Some(StateIndex(117)), None, None, None, None, None, None, None],
   // State 96:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 97:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 98:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 99:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 100:OptionalRepeatModifiersExpression
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 101:)
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 102:GrammarSymbolReference
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 103:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 104:GrammarSymbolReference
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 105::
    [None, None, None, None, None, None, None, Some(StateIndex(119)), Some(StateIndex(32)), None, None, None, None, None, None, None, None, None, Some(StateIndex(34)), Some(StateIndex(33)), Some(StateIndex(35)), Some(StateIndex(36)), Some(StateIndex(39)), Some(StateIndex(37)), None, None, None, None, None, Some(StateIndex(38)), None, None, None, None, None, None],
   // State 106:ProductionMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 107:Const
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 108:IntConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 109:FloatConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 110:BoolConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 111:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 112:TerminalMetaDatas
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 113:}
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 114:,
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(122)), None, Some(StateIndex(90)), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 115:}
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 116:OptionalRepeatModifiers
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 117:OptionalRepeatModifier
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 118:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 119:ProductionRuleRHS
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 120:}
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 121:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 122:TerminalMetaData
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 123:]
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 124:,
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(StateIndex(127)), None, None, None, None, None, None, None],
   // State 125:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 126:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 127:OptionalRepeatModifier
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]   
]

};

impl ParserDefinition for RustemoParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_id: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_id.0].unwrap()
    }
}

pub struct RustemoParser(LRParser<RustemoParserDefinition>);

impl Default for RustemoParser {
    fn default() -> Self {
        Self(LRParser::new(&PARSER_DEFINITION))
    }
}

impl RustemoParser {
    pub fn parse<I, L, B>(mut context: LRContext<I>, lexer: L, mut builder: B) -> RustemoResult<B::Output>
    where
        I: Debug,
        L: Lexer<I, LRContext<I>>,
        B: LRBuilder<I>,
    {
        Self::default().0.parse(context, lexer, builder)
    }

    pub fn parse_str<'i>(input: &'i str) -> RustemoResult<<RustemoBuilder as Builder>::Output> {
        let context = LRContext::new("<str>".to_string(), input);
        let lexer = LRStringLexer::new(&LEXER_DEFINITION);
        let builder = RustemoBuilder::new();
        RustemoParser::default().0.parse(context, lexer, builder) 
    }

    // FIXME: Return/move owned input string with the result.
    // pub fn parse_file<F: AsRef<Path>>(file: F) -> RustemoResult<<RustemoBuilder as Builder>::Output> {
    //    let input = std::fs::read_to_string(file.as_ref())?;
    //    let context = LRContext::new(file.as_ref().to_str().unwrap().to_string(), input);
    //    let lexer = LRStringLexer::new(&LEXER_DEFINITION);
    //    let builder = RustemoBuilder::new();
    //    RustemoParser::default().0.parse(context, lexer, builder) 
    //}
}

pub struct RustemoLexerDefinition {
    terminals: TerminalInfos<TERMINAL_NO>,
    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
}

pub(in crate) static LEXER_DEFINITION: RustemoLexerDefinition = RustemoLexerDefinition {
   terminals: [
    TerminalInfo{
       id: TermIndex(0),
       name: "terminals",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(1),
       name: "import",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(2),
       name: "StrConst",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(3),
       name: ";",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(4),
       name: "as",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(5),
       name: "Name",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(6),
       name: "Action",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(7),
       name: ":",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(8),
       name: "{",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(9),
       name: "}",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(10),
       name: "|",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(11),
       name: "left",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(12),
       name: "reduce",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(13),
       name: "right",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(14),
       name: "shift",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(15),
       name: "dynamic",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(16),
       name: "nops",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(17),
       name: "nopse",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(18),
       name: "IntConst",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(19),
       name: ",",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(20),
       name: "prefer",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(21),
       name: "finish",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(22),
       name: "nofinish",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(23),
       name: "FloatConst",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(24),
       name: "BoolConst",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(25),
       name: "=",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(26),
       name: "?=",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(27),
       name: "(",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(28),
       name: ")",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(29),
       name: "*",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(30),
       name: "*!",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(31),
       name: "+",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(32),
       name: "+!",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(33),
       name: "?",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(34),
       name: "?!",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(35),
       name: "[",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(36),
       name: "]",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(37),
       name: "RegExTerm",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(38),
       name: "WS",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(39),
       name: "/*",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(40),
       name: "*/",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(41),
       name: "CommentLine",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(42),
       name: "NotComment",
       location: None,
   },
    TerminalInfo{
       id: TermIndex(43),
       name: "STOP",
       location: None,
   },   
],

   // Expected terminals/tokens indexed by state id.
   // Sorted by priority.
  terminals_for_state: [
   // State 0:S'
    [Some(0), Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 1:PGFile
    [Some(43), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 2:ProductionRules
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 3:Imports
    [Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 4:terminals
    [Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 5:Import
    [Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 6:import
    [Some(2), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 7:ProductionRuleWithAction
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 8:Action
    [Some(5), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 9:ProductionRule
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 10:Name
    [Some(8), Some(7), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 11:terminals
    [Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 12:ProductionRuleWithAction
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 13:ProductionRules
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 14:Import
    [Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 15:TerminalRules
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 16:TerminalRuleWithAction
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 17:Action
    [Some(5), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 18:TerminalRule
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 19:Name
    [Some(7), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 20:StrConst
    [Some(4), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 21:ProductionRule
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 22::
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 23:{
    [Some(15), Some(12), Some(14), Some(13), Some(17), Some(16), Some(11), Some(5), Some(18), None, None, None, None, None, None],
   // State 24:TerminalRules
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 25:terminals
    [Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 26:TerminalRuleWithAction
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 27:TerminalRule
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 28::
    [Some(8), Some(3), Some(2), Some(37), None, None, None, None, None, None, None, None, None, None, None],
   // State 29:;
    [Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 30:as
    [Some(5), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 31:ProductionRuleRHS
    [Some(10), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 32:Production
    [Some(10), Some(3), Some(28), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 33:Assignments
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 34:Assignment
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 35:PlainAssignment
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 36:BoolAssignment
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 37:GrammarSymbolReference
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 38:GrammarSymbol
    [Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5), None, None],
   // State 39:ProductionGroup
    [Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5), None, None],
   // State 40:(
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 41:Name
    [Some(26), Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(25), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5)],
   // State 42:StrConst
    [Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5), None, None],
   // State 43:ProductionMetaDatas
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 44:ProductionMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 45:left
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 46:reduce
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 47:right
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 48:shift
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 49:dynamic
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 50:nops
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 51:nopse
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 52:IntConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 53:UserMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 54:Name
    [Some(7), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 55:TerminalRules
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 56:Recognizer
    [Some(8), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 57:;
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 58:{
    [Some(22), Some(15), Some(20), Some(21), Some(5), Some(18), None, None, None, None, None, None, None, None, None],
   // State 59:StrConst
    [Some(8), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 60:RegExTerm
    [Some(8), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 61:Name
    [Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 62:;
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 63:|
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 64:{
    [Some(15), Some(12), Some(14), Some(13), Some(17), Some(16), Some(11), Some(5), Some(18), None, None, None, None, None, None],
   // State 65:Assignment
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 66:OptRepeatOperator
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 67:RepeatOperator
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 68:*
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 69:*!
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 70:+
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 71:+!
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 72:?
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 73:?!
    [Some(10), Some(8), Some(35), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None],
   // State 74:OptRepeatOperator
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 75:ProductionRuleRHS
    [Some(10), Some(28), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 76:?=
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 77:=
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 78:}
    [Some(7), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 79:,
    [Some(15), Some(12), Some(14), Some(13), Some(17), Some(16), Some(11), Some(5), Some(18), None, None, None, None, None, None],
   // State 80::
    [Some(2), Some(18), Some(23), Some(24), None, None, None, None, None, None, None, None, None, None, None],
   // State 81:;
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 82:{
    [Some(22), Some(15), Some(20), Some(21), Some(5), Some(18), None, None, None, None, None, None, None, None, None],
   // State 83:TerminalMetaDatas
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 84:TerminalMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 85:prefer
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 86:finish
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 87:nofinish
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 88:dynamic
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 89:IntConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 90:UserMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 91:;
    [Some(1), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 92:Production
    [Some(10), Some(3), Some(28), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 93:ProductionMetaDatas
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 94:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 95:[
    [Some(5), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 96:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 97:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 98:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 99:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 100:OptionalRepeatModifiersExpression
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 101:)
    [Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5), None, None],
   // State 102:GrammarSymbolReference
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 103:Name
    [Some(34), Some(32), Some(30), Some(10), Some(8), Some(33), Some(3), Some(31), Some(29), Some(28), Some(27), Some(2), Some(5), None, None],
   // State 104:GrammarSymbolReference
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 105::
    [Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 106:ProductionMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 107:Const
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 108:IntConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 109:FloatConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 110:BoolConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 111:StrConst
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 112:TerminalMetaDatas
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 113:}
    [Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 114:,
    [Some(22), Some(15), Some(20), Some(21), Some(5), Some(18), None, None, None, None, None, None, None, None, None],
   // State 115:}
    [Some(10), Some(3), Some(28), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 116:OptionalRepeatModifiers
    [Some(36), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 117:OptionalRepeatModifier
    [Some(36), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 118:Name
    [Some(36), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 119:ProductionRuleRHS
    [Some(10), Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 120:}
    [Some(3), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 121:;
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 122:TerminalMetaData
    [Some(9), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 123:]
    [Some(10), Some(8), Some(3), Some(28), Some(27), Some(2), Some(5), None, None, None, None, None, None, None, None],
   // State 124:,
    [Some(5), None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 125:;
    [Some(0), Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None],
   // State 126:;
    [Some(43), Some(5), Some(6), None, None, None, None, None, None, None, None, None, None, None, None],
   // State 127:OptionalRepeatModifier
    [Some(36), Some(19), None, None, None, None, None, None, None, None, None, None, None, None, None]   
],

   recognizers: [
       // 0:terminals
       |input: &str| {
           logn!("Recognizing <terminals> -- ");
           if input.starts_with("terminals"){
              log!("recognized");
              Some("terminals")
           } else {
              log!("not recognized");
              None
           }
       },
       // 1:import
       |input: &str| {
           logn!("Recognizing <import> -- ");
           if input.starts_with("import"){
              log!("recognized");
              Some("import")
           } else {
              log!("not recognized");
              None
           }
       },
       // 2:StrConst
       |input: &str| {
           logn!("Recognizing <StrConst> -- ");
           let regex = Regex::new(r#"^((?s)('[^'\\]*(?:\\.[^'\\]*)*')|("[^"\\]*(?:\\.[^"\\]*)*"))"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 3:;
       |input: &str| {
           logn!("Recognizing <;> -- ");
           if input.starts_with(";"){
              log!("recognized");
              Some(";")
           } else {
              log!("not recognized");
              None
           }
       },
       // 4:as
       |input: &str| {
           logn!("Recognizing <as> -- ");
           if input.starts_with("as"){
              log!("recognized");
              Some("as")
           } else {
              log!("not recognized");
              None
           }
       },
       // 5:Name
       |input: &str| {
           logn!("Recognizing <Name> -- ");
           let regex = Regex::new(r#"^([a-zA-Z_][a-zA-Z0-9_\.]*)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 6:Action
       |input: &str| {
           logn!("Recognizing <Action> -- ");
           let regex = Regex::new(r#"^(@[a-zA-Z0-9_]+)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 7::
       |input: &str| {
           logn!("Recognizing <:> -- ");
           if input.starts_with(":"){
              log!("recognized");
              Some(":")
           } else {
              log!("not recognized");
              None
           }
       },
       // 8:{
       |input: &str| {
           logn!("Recognizing <{{> -- ");
           if input.starts_with("{"){
              log!("recognized");
              Some("{")
           } else {
              log!("not recognized");
              None
           }
       },
       // 9:}
       |input: &str| {
           logn!("Recognizing <}}> -- ");
           if input.starts_with("}"){
              log!("recognized");
              Some("}")
           } else {
              log!("not recognized");
              None
           }
       },
       // 10:|
       |input: &str| {
           logn!("Recognizing <|> -- ");
           if input.starts_with("|"){
              log!("recognized");
              Some("|")
           } else {
              log!("not recognized");
              None
           }
       },
       // 11:left
       |input: &str| {
           logn!("Recognizing <left> -- ");
           if input.starts_with("left"){
              log!("recognized");
              Some("left")
           } else {
              log!("not recognized");
              None
           }
       },
       // 12:reduce
       |input: &str| {
           logn!("Recognizing <reduce> -- ");
           if input.starts_with("reduce"){
              log!("recognized");
              Some("reduce")
           } else {
              log!("not recognized");
              None
           }
       },
       // 13:right
       |input: &str| {
           logn!("Recognizing <right> -- ");
           if input.starts_with("right"){
              log!("recognized");
              Some("right")
           } else {
              log!("not recognized");
              None
           }
       },
       // 14:shift
       |input: &str| {
           logn!("Recognizing <shift> -- ");
           if input.starts_with("shift"){
              log!("recognized");
              Some("shift")
           } else {
              log!("not recognized");
              None
           }
       },
       // 15:dynamic
       |input: &str| {
           logn!("Recognizing <dynamic> -- ");
           if input.starts_with("dynamic"){
              log!("recognized");
              Some("dynamic")
           } else {
              log!("not recognized");
              None
           }
       },
       // 16:nops
       |input: &str| {
           logn!("Recognizing <nops> -- ");
           if input.starts_with("nops"){
              log!("recognized");
              Some("nops")
           } else {
              log!("not recognized");
              None
           }
       },
       // 17:nopse
       |input: &str| {
           logn!("Recognizing <nopse> -- ");
           if input.starts_with("nopse"){
              log!("recognized");
              Some("nopse")
           } else {
              log!("not recognized");
              None
           }
       },
       // 18:IntConst
       |input: &str| {
           logn!("Recognizing <IntConst> -- ");
           let regex = Regex::new(r#"^(\d+)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 19:,
       |input: &str| {
           logn!("Recognizing <,> -- ");
           if input.starts_with(","){
              log!("recognized");
              Some(",")
           } else {
              log!("not recognized");
              None
           }
       },
       // 20:prefer
       |input: &str| {
           logn!("Recognizing <prefer> -- ");
           if input.starts_with("prefer"){
              log!("recognized");
              Some("prefer")
           } else {
              log!("not recognized");
              None
           }
       },
       // 21:finish
       |input: &str| {
           logn!("Recognizing <finish> -- ");
           if input.starts_with("finish"){
              log!("recognized");
              Some("finish")
           } else {
              log!("not recognized");
              None
           }
       },
       // 22:nofinish
       |input: &str| {
           logn!("Recognizing <nofinish> -- ");
           if input.starts_with("nofinish"){
              log!("recognized");
              Some("nofinish")
           } else {
              log!("not recognized");
              None
           }
       },
       // 23:FloatConst
       |input: &str| {
           logn!("Recognizing <FloatConst> -- ");
           let regex = Regex::new(r#"^([+-]?(\d+\.\d*|\.\d+)([eE][+-]?\d+)?(?<=[\w\.])(?![\w\.]))"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 24:BoolConst
       |input: &str| {
           logn!("Recognizing <BoolConst> -- ");
           let regex = Regex::new(r#"^(true|false)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 25:=
       |input: &str| {
           logn!("Recognizing <=> -- ");
           if input.starts_with("="){
              log!("recognized");
              Some("=")
           } else {
              log!("not recognized");
              None
           }
       },
       // 26:?=
       |input: &str| {
           logn!("Recognizing <?=> -- ");
           if input.starts_with("?="){
              log!("recognized");
              Some("?=")
           } else {
              log!("not recognized");
              None
           }
       },
       // 27:(
       |input: &str| {
           logn!("Recognizing <(> -- ");
           if input.starts_with("("){
              log!("recognized");
              Some("(")
           } else {
              log!("not recognized");
              None
           }
       },
       // 28:)
       |input: &str| {
           logn!("Recognizing <)> -- ");
           if input.starts_with(")"){
              log!("recognized");
              Some(")")
           } else {
              log!("not recognized");
              None
           }
       },
       // 29:*
       |input: &str| {
           logn!("Recognizing <*> -- ");
           if input.starts_with("*"){
              log!("recognized");
              Some("*")
           } else {
              log!("not recognized");
              None
           }
       },
       // 30:*!
       |input: &str| {
           logn!("Recognizing <*!> -- ");
           if input.starts_with("*!"){
              log!("recognized");
              Some("*!")
           } else {
              log!("not recognized");
              None
           }
       },
       // 31:+
       |input: &str| {
           logn!("Recognizing <+> -- ");
           if input.starts_with("+"){
              log!("recognized");
              Some("+")
           } else {
              log!("not recognized");
              None
           }
       },
       // 32:+!
       |input: &str| {
           logn!("Recognizing <+!> -- ");
           if input.starts_with("+!"){
              log!("recognized");
              Some("+!")
           } else {
              log!("not recognized");
              None
           }
       },
       // 33:?
       |input: &str| {
           logn!("Recognizing <?> -- ");
           if input.starts_with("?"){
              log!("recognized");
              Some("?")
           } else {
              log!("not recognized");
              None
           }
       },
       // 34:?!
       |input: &str| {
           logn!("Recognizing <?!> -- ");
           if input.starts_with("?!"){
              log!("recognized");
              Some("?!")
           } else {
              log!("not recognized");
              None
           }
       },
       // 35:[
       |input: &str| {
           logn!("Recognizing <[> -- ");
           if input.starts_with("["){
              log!("recognized");
              Some("[")
           } else {
              log!("not recognized");
              None
           }
       },
       // 36:]
       |input: &str| {
           logn!("Recognizing <]> -- ");
           if input.starts_with("]"){
              log!("recognized");
              Some("]")
           } else {
              log!("not recognized");
              None
           }
       },
       // 37:RegExTerm
       |input: &str| {
           logn!("Recognizing <RegExTerm> -- ");
           let regex = Regex::new(r#"^(/(\\.|[^/\\])*/)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 38:WS
       |input: &str| {
           logn!("Recognizing <WS> -- ");
           let regex = Regex::new(r#"^(\s+)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 39:/*
       |input: &str| {
           logn!("Recognizing </*> -- ");
           if input.starts_with("/*"){
              log!("recognized");
              Some("/*")
           } else {
              log!("not recognized");
              None
           }
       },
       // 40:*/
       |input: &str| {
           logn!("Recognizing <*/> -- ");
           if input.starts_with("*/"){
              log!("recognized");
              Some("*/")
           } else {
              log!("not recognized");
              None
           }
       },
       // 41:CommentLine
       |input: &str| {
           logn!("Recognizing <CommentLine> -- ");
           let regex = Regex::new(r#"^(//.*)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 42:NotComment
       |input: &str| {
           logn!("Recognizing <NotComment> -- ");
           let regex = Regex::new(r#"^(((\*[^/])|[^\s*/]|/[^\*])+)"#).unwrap();
           let match_str = regex.find(input);
           match match_str {
               Some(x) => {
                   let x_str = x.as_str();
                   log!("recognized <{}>", x_str);
                   Some(x_str)
               },
               None => {
                   log!("not recognized");
                   None
               }
           }
       },
       // 43:STOP
       |input: &str| {
           logn!("Recognizing <STOP> -- ");
           if input.is_empty() { Some("") } else { None }
       },
   ]
};

impl LexerDefinition for RustemoLexerDefinition {
    type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;

    fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {
            RecognizerIterator {
                terminals: &LEXER_DEFINITION.terminals,
                terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                recognizers: &LEXER_DEFINITION.recognizers,
                index: 0
            }
    }
}

pub struct RustemoBuilder {
    res_stack: Vec<Symbol>,
}

impl Builder for RustemoBuilder
{
    type Output = Symbol;

    fn new() -> Self {
        RustemoBuilder {
            res_stack: vec![],
        }
    }

    fn get_result(&mut self) -> RustemoResult<Self::Output> {
       Ok(self.res_stack.pop().unwrap())
    }
}

impl<'i> LRBuilder<&'i str> for RustemoBuilder
{
    fn shift_action(&mut self, term_idx: TermIndex, token: Token<&'i str>) {
        let termval = match TermKind::try_from(term_idx.0).unwrap() {
            TermKind::Terminals => Terminal::Terminals,
            TermKind::Import => Terminal::Import,
            TermKind::StrConst => Terminal::StrConst(str_const(token)),
            TermKind::SemiColon => Terminal::SemiColon,
            TermKind::As => Terminal::As,
            TermKind::Name => Terminal::Name(name(token)),
            TermKind::Action => Terminal::Action(action(token)),
            TermKind::Colon => Terminal::Colon,
            TermKind::OBrace => Terminal::OBrace,
            TermKind::CBrace => Terminal::CBrace,
            TermKind::Bar => Terminal::Bar,
            TermKind::Left => Terminal::Left,
            TermKind::Reduce => Terminal::Reduce,
            TermKind::Right => Terminal::Right,
            TermKind::Shift => Terminal::Shift,
            TermKind::Dynamic => Terminal::Dynamic,
            TermKind::Nops => Terminal::Nops,
            TermKind::Nopse => Terminal::Nopse,
            TermKind::IntConst => Terminal::IntConst(int_const(token)),
            TermKind::Comma => Terminal::Comma,
            TermKind::Prefer => Terminal::Prefer,
            TermKind::Finish => Terminal::Finish,
            TermKind::Nofinish => Terminal::Nofinish,
            TermKind::FloatConst => Terminal::FloatConst(float_const(token)),
            TermKind::BoolConst => Terminal::BoolConst(bool_const(token)),
            TermKind::Equals => Terminal::Equals,
            TermKind::QEquals => Terminal::QEquals,
            TermKind::OBracket => Terminal::OBracket,
            TermKind::CBracket => Terminal::CBracket,
            TermKind::Asterisk => Terminal::Asterisk,
            TermKind::AsteriskGready => Terminal::AsteriskGready,
            TermKind::Plus => Terminal::Plus,
            TermKind::PlusGready => Terminal::PlusGready,
            TermKind::Question => Terminal::Question,
            TermKind::QuestionGready => Terminal::QuestionGready,
            TermKind::OSquare => Terminal::OSquare,
            TermKind::CSquare => Terminal::CSquare,
            TermKind::RegExTerm => Terminal::RegExTerm(reg_ex_term(token)),
            TermKind::WS => Terminal::WS(ws(token)),
            TermKind::OComment => Terminal::OComment,
            TermKind::CComment => Terminal::CComment,
            TermKind::CommentLine => Terminal::CommentLine(comment_line(token)),
            TermKind::NotComment => Terminal::NotComment(not_comment(token)),
            TermKind::STOP => Terminal::STOP(stop(token)),
        };
        self.res_stack.push(Symbol::Terminal(termval));
    }

    fn reduce_action(&mut self, prod_idx: ProdIndex, prod_len: usize, _prod_str: &'static str) {
        let prod = match ProdKind::try_from(prod_idx.0).unwrap() {
            ProdKind::PGFileP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRules(p0))) => NonTerminal::PGFile(pgfile_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::PGFileP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Imports(p0)), Symbol::NonTerminal(NonTerminal::ProductionRules(p1))) => NonTerminal::PGFile(pgfile_p1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::PGFileP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRules(p0)), _, Symbol::NonTerminal(NonTerminal::TerminalRules(p1))) => NonTerminal::PGFile(pgfile_p2(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::PGFileP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-4).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Imports(p0)), Symbol::NonTerminal(NonTerminal::ProductionRules(p1)), _, Symbol::NonTerminal(NonTerminal::TerminalRules(p2))) => NonTerminal::PGFile(pgfile_p3(p0, p1, p2)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::PGFileP4 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::TerminalRules(p0))) => NonTerminal::PGFile(pgfile_p4(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ImportsP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Imports(p0)), Symbol::NonTerminal(NonTerminal::Import(p1))) => NonTerminal::Imports(imports_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ImportsP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Import(p0))) => NonTerminal::Imports(imports_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ImportP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::Terminal(Terminal::StrConst(p0)), _) => NonTerminal::Import(import_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ImportP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-5).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::Terminal(Terminal::StrConst(p0)), _, Symbol::Terminal(Terminal::Name(p1)), _) => NonTerminal::Import(import_p1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRulesP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRules(p0)), Symbol::NonTerminal(NonTerminal::ProductionRuleWithAction(p1))) => NonTerminal::ProductionRules(production_rules_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRulesP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRuleWithAction(p0))) => NonTerminal::ProductionRules(production_rules_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleWithActionP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Action(p0)), Symbol::NonTerminal(NonTerminal::ProductionRule(p1))) => NonTerminal::ProductionRuleWithAction(production_rule_with_action_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleWithActionP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRule(p0))) => NonTerminal::ProductionRuleWithAction(production_rule_with_action_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-4).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::ProductionRuleRHS(p1)), _) => NonTerminal::ProductionRule(production_rule_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-7).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::ProductionMetaDatas(p1)), _, _, Symbol::NonTerminal(NonTerminal::ProductionRuleRHS(p2)), _) => NonTerminal::ProductionRule(production_rule_p1(p0, p1, p2)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleRHSP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionRuleRHS(p0)), _, Symbol::NonTerminal(NonTerminal::Production(p1))) => NonTerminal::ProductionRuleRHS(production_rule_rhsp0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionRuleRHSP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Production(p0))) => NonTerminal::ProductionRuleRHS(production_rule_rhsp1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Assignments(p0))) => NonTerminal::Production(production_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-4).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Assignments(p0)), _, Symbol::NonTerminal(NonTerminal::ProductionMetaDatas(p1)), _) => NonTerminal::Production(production_p1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRulesP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::TerminalRules(p0)), Symbol::NonTerminal(NonTerminal::TerminalRuleWithAction(p1))) => NonTerminal::TerminalRules(terminal_rules_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRulesP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::TerminalRuleWithAction(p0))) => NonTerminal::TerminalRules(terminal_rules_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleWithActionP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Action(p0)), Symbol::NonTerminal(NonTerminal::TerminalRule(p1))) => NonTerminal::TerminalRuleWithAction(terminal_rule_with_action_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleWithActionP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::TerminalRule(p0))) => NonTerminal::TerminalRuleWithAction(terminal_rule_with_action_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-4).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::Recognizer(p1)), _) => NonTerminal::TerminalRule(terminal_rule_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, _) => NonTerminal::TerminalRule(terminal_rule_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-7).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::Recognizer(p1)), _, Symbol::NonTerminal(NonTerminal::TerminalMetaDatas(p2)), _, _) => NonTerminal::TerminalRule(terminal_rule_p2(p0, p1, p2)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalRuleP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-6).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, _, Symbol::NonTerminal(NonTerminal::TerminalMetaDatas(p1)), _, _) => NonTerminal::TerminalRule(terminal_rule_p3(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p0()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p1()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p2()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p3()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP4 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p4()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP5 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p5()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP6 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::ProductionMetaData(production_meta_data_p6()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP7 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::IntConst(p0))) => NonTerminal::ProductionMetaData(production_meta_data_p7(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDataP8 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::UserMetaData(p0))) => NonTerminal::ProductionMetaData(production_meta_data_p8(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDatasP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionMetaDatas(p0)), _, Symbol::NonTerminal(NonTerminal::ProductionMetaData(p1))) => NonTerminal::ProductionMetaDatas(production_meta_datas_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionMetaDatasP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionMetaData(p0))) => NonTerminal::ProductionMetaDatas(production_meta_datas_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::TerminalMetaData(terminal_meta_data_p0()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::TerminalMetaData(terminal_meta_data_p1()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::TerminalMetaData(terminal_meta_data_p2()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (_) => NonTerminal::TerminalMetaData(terminal_meta_data_p3()),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP4 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::IntConst(p0))) => NonTerminal::TerminalMetaData(terminal_meta_data_p4(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDataP5 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::UserMetaData(p0))) => NonTerminal::TerminalMetaData(terminal_meta_data_p5(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDatasP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::TerminalMetaDatas(p0)), _, Symbol::NonTerminal(NonTerminal::TerminalMetaData(p1))) => NonTerminal::TerminalMetaDatas(terminal_meta_datas_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TerminalMetaDatasP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::TerminalMetaData(p0))) => NonTerminal::TerminalMetaDatas(terminal_meta_datas_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::UserMetaDataP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::Const(p1))) => NonTerminal::UserMetaData(user_meta_data_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ConstP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::IntConst(p0))) => NonTerminal::Const(const_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ConstP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::FloatConst(p0))) => NonTerminal::Const(const_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ConstP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::BoolConst(p0))) => NonTerminal::Const(const_p2(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ConstP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::StrConst(p0))) => NonTerminal::Const(const_p3(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::AssignmentP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::PlainAssignment(p0))) => NonTerminal::Assignment(assignment_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::AssignmentP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::BoolAssignment(p0))) => NonTerminal::Assignment(assignment_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::AssignmentP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::GrammarSymbolReference(p0))) => NonTerminal::Assignment(assignment_p2(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::AssignmentsP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Assignments(p0)), Symbol::NonTerminal(NonTerminal::Assignment(p1))) => NonTerminal::Assignments(assignments_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::AssignmentsP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Assignment(p0))) => NonTerminal::Assignments(assignments_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::PlainAssignmentP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::GrammarSymbolReference(p1))) => NonTerminal::PlainAssignment(plain_assignment_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::BoolAssignmentP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0)), _, Symbol::NonTerminal(NonTerminal::GrammarSymbolReference(p1))) => NonTerminal::BoolAssignment(bool_assignment_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::ProductionGroupP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::ProductionRuleRHS(p0)), _) => NonTerminal::ProductionGroup(production_group_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::GrammarSymbolReferenceP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::GrammarSymbol(p0)), Symbol::NonTerminal(NonTerminal::OptRepeatOperator(p1))) => NonTerminal::GrammarSymbolReference(grammar_symbol_reference_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::GrammarSymbolReferenceP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::ProductionGroup(p0)), Symbol::NonTerminal(NonTerminal::OptRepeatOperator(p1))) => NonTerminal::GrammarSymbolReference(grammar_symbol_reference_p1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptRepeatOperatorP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::RepeatOperator(p0))) => NonTerminal::OptRepeatOperator(opt_repeat_operator_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptRepeatOperatorP1 => NonTerminal::OptRepeatOperator(opt_repeat_operator_p1()),
            ProdKind::RepeatOperatorP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RepeatOperatorP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RepeatOperatorP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p2(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RepeatOperatorP3 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p3(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RepeatOperatorP4 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p4(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RepeatOperatorP5 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiersExpression(p0))) => NonTerminal::RepeatOperator(repeat_operator_p5(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptionalRepeatModifiersExpressionP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiers(p0)), _) => NonTerminal::OptionalRepeatModifiersExpression(optional_repeat_modifiers_expression_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptionalRepeatModifiersExpressionP1 => NonTerminal::OptionalRepeatModifiersExpression(optional_repeat_modifiers_expression_p1()),
            ProdKind::OptionalRepeatModifiersP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::OptionalRepeatModifiers(p0)), _, Symbol::NonTerminal(NonTerminal::OptionalRepeatModifier(p1))) => NonTerminal::OptionalRepeatModifiers(optional_repeat_modifiers_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptionalRepeatModifiersP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::OptionalRepeatModifier(p0))) => NonTerminal::OptionalRepeatModifiers(optional_repeat_modifiers_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::OptionalRepeatModifierP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0))) => NonTerminal::OptionalRepeatModifier(optional_repeat_modifier_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::GrammarSymbolP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::Name(p0))) => NonTerminal::GrammarSymbol(grammar_symbol_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::GrammarSymbolP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::StrConst(p0))) => NonTerminal::GrammarSymbol(grammar_symbol_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RecognizerP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::StrConst(p0))) => NonTerminal::Recognizer(recognizer_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::RecognizerP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::RegExTerm(p0))) => NonTerminal::Recognizer(recognizer_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::LAYOUTP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::LAYOUTITEM(p0))) => NonTerminal::LAYOUT(layoutp0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::LAYOUTP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::LAYOUT(p0)), Symbol::NonTerminal(NonTerminal::LAYOUTITEM(p1))) => NonTerminal::LAYOUT(layoutp1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::LAYOUTP2 => NonTerminal::LAYOUT(layoutp2()),
            ProdKind::LAYOUTITEMP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::WS(p0))) => NonTerminal::LAYOUTITEM(layoutitemp0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::LAYOUTITEMP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Comment(p0))) => NonTerminal::LAYOUTITEM(layoutitemp1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CommentP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (_, Symbol::NonTerminal(NonTerminal::CORNCS(p0)), _) => NonTerminal::Comment(comment_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CommentP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::CommentLine(p0))) => NonTerminal::Comment(comment_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CORNCSP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::CORNC(p0))) => NonTerminal::CORNCS(corncsp0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CORNCSP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-2).into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::CORNCS(p0)), Symbol::NonTerminal(NonTerminal::CORNC(p1))) => NonTerminal::CORNCS(corncsp1(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CORNCSP2 => NonTerminal::CORNCS(corncsp2()),
            ProdKind::CORNCP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::NonTerminal(NonTerminal::Comment(p0))) => NonTerminal::CORNC(corncp0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CORNCP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::NotComment(p0))) => NonTerminal::CORNC(corncp1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::CORNCP2 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match (i.next().unwrap()) {
                    (Symbol::Terminal(Terminal::WS(p0))) => NonTerminal::CORNC(corncp2(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }

}

