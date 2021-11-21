// Generated on 2022-02-03 00:00:05.991423 from bootstrap.py. Do not edit!

use regex::Regex;
use std::convert::TryFrom;

use std::marker::PhantomData;
use crate::lexer::{Lexer, Token};
use crate::parser::ParserDefinition;
use crate::builder::Builder;
use crate::grammar::{TerminalInfo, TerminalInfos, TerminalsState};
use crate::parser::Action::{self, Shift, Reduce, Accept, Error};
use crate::debug::{log, logn};
use super::parser::GrammarLexer;
use super::rustemo_types::{TermKind, ProdKind, Terminal, NonTerminal, Symbol};

use super::types::*;

use super::lexer::{LexerDefinition, RecognizerIterator};
const TERMINAL_NO: usize = 44;
const NONTERMINAL_NO: usize = 36;
const STATE_NO: usize = 128;
const MAX_ACTIONS: usize = 15;

pub struct RustemoParserDefinition {
    actions: [[Action; TERMINAL_NO]; STATE_NO],
    gotos: [[Option<usize>; NONTERMINAL_NO]; STATE_NO]
}

pub(in crate::lang) static PARSER_DEFINITION: RustemoParserDefinition = RustemoParserDefinition {
   actions: [
   // State 0:S'
    [Shift(4, 0), Shift(6, 1), Error, Error, Error, Shift(10, 5), Shift(8, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 1:PGFile
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Accept],
   // State 2:ProductionRules
    [Shift(11, 0), Error, Error, Error, Error, Shift(10, 5), Shift(8, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(1, 1, 1, "1: PGFile = ProductionRules")],
   // State 3:Imports
    [Error, Shift(6, 1), Error, Error, Error, Shift(10, 5), Shift(8, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 4:terminals
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 5:Import
    [Error, Reduce(7, 1, 2, "7: Imports = Import"), Error, Error, Error, Reduce(7, 1, 2, "7: Imports = Import"), Reduce(7, 1, 2, "7: Imports = Import"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 6:import
    [Error, Error, Shift(20, 2), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 7:ProductionRuleWithAction
    [Reduce(11, 1, 4, "11: ProductionRules = ProductionRuleWithAction"), Error, Error, Error, Error, Reduce(11, 1, 4, "11: ProductionRules = ProductionRuleWithAction"), Reduce(11, 1, 4, "11: ProductionRules = ProductionRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(11, 1, 4, "11: ProductionRules = ProductionRuleWithAction")],
   // State 8:Action
    [Error, Error, Error, Error, Error, Shift(10, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 9:ProductionRule
    [Reduce(13, 1, 5, "13: ProductionRuleWithAction = ProductionRule"), Error, Error, Error, Error, Reduce(13, 1, 5, "13: ProductionRuleWithAction = ProductionRule"), Reduce(13, 1, 5, "13: ProductionRuleWithAction = ProductionRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(13, 1, 5, "13: ProductionRuleWithAction = ProductionRule")],
   // State 10:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(22, 7), Shift(23, 8), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 11:terminals
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 12:ProductionRuleWithAction
    [Reduce(10, 2, 4, "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Error, Error, Error, Error, Reduce(10, 2, 4, "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Reduce(10, 2, 4, "10: ProductionRules = ProductionRules ProductionRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(10, 2, 4, "10: ProductionRules = ProductionRules ProductionRuleWithAction")],
   // State 13:ProductionRules
    [Shift(25, 0), Error, Error, Error, Error, Shift(10, 5), Shift(8, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(2, 2, 1, "2: PGFile = Imports ProductionRules")],
   // State 14:Import
    [Error, Reduce(6, 2, 2, "6: Imports = Imports Import"), Error, Error, Error, Reduce(6, 2, 2, "6: Imports = Imports Import"), Reduce(6, 2, 2, "6: Imports = Imports Import"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 15:TerminalRules
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(5, 2, 1, "5: PGFile = terminals TerminalRules")],
   // State 16:TerminalRuleWithAction
    [Error, Error, Error, Error, Error, Reduce(21, 1, 9, "21: TerminalRules = TerminalRuleWithAction"), Reduce(21, 1, 9, "21: TerminalRules = TerminalRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(21, 1, 9, "21: TerminalRules = TerminalRuleWithAction")],
   // State 17:Action
    [Error, Error, Error, Error, Error, Shift(19, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 18:TerminalRule
    [Error, Error, Error, Error, Error, Reduce(23, 1, 10, "23: TerminalRuleWithAction = TerminalRule"), Reduce(23, 1, 10, "23: TerminalRuleWithAction = TerminalRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(23, 1, 10, "23: TerminalRuleWithAction = TerminalRule")],
   // State 19:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(28, 7), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 20:StrConst
    [Error, Error, Error, Shift(29, 3), Shift(30, 4), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 21:ProductionRule
    [Reduce(12, 2, 5, "12: ProductionRuleWithAction = Action ProductionRule"), Error, Error, Error, Error, Reduce(12, 2, 5, "12: ProductionRuleWithAction = Action ProductionRule"), Reduce(12, 2, 5, "12: ProductionRuleWithAction = Action ProductionRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(12, 2, 5, "12: ProductionRuleWithAction = Action ProductionRule")],
   // State 22::
    [Error, Error, Shift(42, 2), Error, Error, Shift(41, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 23:{
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Shift(45, 11), Shift(46, 12), Shift(47, 13), Shift(48, 14), Shift(49, 15), Shift(50, 16), Shift(51, 17), Shift(52, 18), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 24:TerminalRules
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(3, 3, 1, "3: PGFile = ProductionRules terminals TerminalRules")],
   // State 25:terminals
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 26:TerminalRuleWithAction
    [Error, Error, Error, Error, Error, Reduce(20, 2, 9, "20: TerminalRules = TerminalRules TerminalRuleWithAction"), Reduce(20, 2, 9, "20: TerminalRules = TerminalRules TerminalRuleWithAction"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(20, 2, 9, "20: TerminalRules = TerminalRules TerminalRuleWithAction")],
   // State 27:TerminalRule
    [Error, Error, Error, Error, Error, Reduce(22, 2, 10, "22: TerminalRuleWithAction = Action TerminalRule"), Reduce(22, 2, 10, "22: TerminalRuleWithAction = Action TerminalRule"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(22, 2, 10, "22: TerminalRuleWithAction = Action TerminalRule")],
   // State 28::
    [Error, Error, Shift(59, 2), Shift(57, 3), Error, Error, Error, Error, Shift(58, 8), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(60, 37), Error, Error, Error, Error, Error, Error],
   // State 29:;
    [Error, Reduce(8, 3, 3, "8: Import = import StrConst ;"), Error, Error, Error, Reduce(8, 3, 3, "8: Import = import StrConst ;"), Reduce(8, 3, 3, "8: Import = import StrConst ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 30:as
    [Error, Error, Error, Error, Error, Shift(61, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 31:ProductionRuleRHS
    [Error, Error, Error, Shift(62, 3), Error, Error, Error, Error, Error, Error, Shift(63, 10), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 32:Production
    [Error, Error, Error, Reduce(17, 1, 7, "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Reduce(17, 1, 7, "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(17, 1, 7, "17: ProductionRuleRHS = Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 33:Assignments
    [Error, Error, Shift(42, 2), Reduce(18, 1, 8, "18: Production = Assignments"), Error, Shift(41, 5), Error, Error, Shift(64, 8), Error, Reduce(18, 1, 8, "18: Production = Assignments"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Reduce(18, 1, 8, "18: Production = Assignments"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 34:Assignment
    [Error, Error, Reduce(56, 1, 19, "56: Assignments = Assignment"), Reduce(56, 1, 19, "56: Assignments = Assignment"), Error, Reduce(56, 1, 19, "56: Assignments = Assignment"), Error, Error, Reduce(56, 1, 19, "56: Assignments = Assignment"), Error, Reduce(56, 1, 19, "56: Assignments = Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(56, 1, 19, "56: Assignments = Assignment"), Reduce(56, 1, 19, "56: Assignments = Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 35:PlainAssignment
    [Error, Error, Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Error, Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Error, Error, Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Error, Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Reduce(52, 1, 18, "52: Assignment = PlainAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 36:BoolAssignment
    [Error, Error, Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Error, Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Error, Error, Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Error, Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Reduce(53, 1, 18, "53: Assignment = BoolAssignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 37:GrammarSymbolReference
    [Error, Error, Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Error, Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Error, Error, Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Error, Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Reduce(54, 1, 18, "54: Assignment = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 38:GrammarSymbol
    [Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Shift(68, 29), Shift(69, 30), Shift(70, 31), Shift(71, 32), Shift(72, 33), Shift(73, 34), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 39:ProductionGroup
    [Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Reduce(63, 0, 24, "63: OptRepeatOperator = EMPTY"), Shift(68, 29), Shift(69, 30), Shift(70, 31), Shift(71, 32), Shift(72, 33), Shift(73, 34), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 40:(
    [Error, Error, Shift(42, 2), Error, Error, Shift(41, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 41:Name
    [Error, Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(77, 25), Shift(76, 26), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 42:StrConst
    [Error, Error, Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Error, Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Error, Error, Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Error, Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Reduce(76, 1, 29, "76: GrammarSymbol = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 43:ProductionMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(78, 9), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(79, 19), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 44:ProductionMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(38, 1, 13, "38: ProductionMetaDatas = ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(38, 1, 13, "38: ProductionMetaDatas = ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 45:left
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(28, 1, 12, "28: ProductionMetaData = left"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(28, 1, 12, "28: ProductionMetaData = left"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 46:reduce
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(29, 1, 12, "29: ProductionMetaData = reduce"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(29, 1, 12, "29: ProductionMetaData = reduce"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 47:right
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(30, 1, 12, "30: ProductionMetaData = right"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(30, 1, 12, "30: ProductionMetaData = right"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 48:shift
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(31, 1, 12, "31: ProductionMetaData = shift"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(31, 1, 12, "31: ProductionMetaData = shift"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 49:dynamic
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(32, 1, 12, "32: ProductionMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(32, 1, 12, "32: ProductionMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 50:nops
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(33, 1, 12, "33: ProductionMetaData = nops"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(33, 1, 12, "33: ProductionMetaData = nops"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 51:nopse
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(34, 1, 12, "34: ProductionMetaData = nopse"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(34, 1, 12, "34: ProductionMetaData = nopse"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 52:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(35, 1, 12, "35: ProductionMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(35, 1, 12, "35: ProductionMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 53:UserMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(36, 1, 12, "36: ProductionMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(36, 1, 12, "36: ProductionMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 54:Name
    [Error, Error, Error, Error, Error, Error, Error, Shift(80, 7), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 55:TerminalRules
    [Error, Error, Error, Error, Error, Shift(19, 5), Shift(17, 6), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(4, 4, 1, "4: PGFile = Imports ProductionRules terminals TerminalRules")],
   // State 56:Recognizer
    [Error, Error, Error, Shift(81, 3), Error, Error, Error, Error, Shift(82, 8), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 57:;
    [Error, Error, Error, Error, Error, Reduce(25, 3, 11, "25: TerminalRule = Name : ;"), Reduce(25, 3, 11, "25: TerminalRule = Name : ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(25, 3, 11, "25: TerminalRule = Name : ;")],
   // State 58:{
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(88, 15), Error, Error, Shift(89, 18), Error, Shift(85, 20), Shift(86, 21), Shift(87, 22), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 59:StrConst
    [Error, Error, Error, Reduce(77, 1, 30, "77: Recognizer = StrConst"), Error, Error, Error, Error, Reduce(77, 1, 30, "77: Recognizer = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 60:RegExTerm
    [Error, Error, Error, Reduce(78, 1, 30, "78: Recognizer = RegExTerm"), Error, Error, Error, Error, Reduce(78, 1, 30, "78: Recognizer = RegExTerm"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 61:Name
    [Error, Error, Error, Shift(91, 3), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 62:;
    [Reduce(14, 4, 6, "14: ProductionRule = Name : ProductionRuleRHS ;"), Error, Error, Error, Error, Reduce(14, 4, 6, "14: ProductionRule = Name : ProductionRuleRHS ;"), Reduce(14, 4, 6, "14: ProductionRule = Name : ProductionRuleRHS ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(14, 4, 6, "14: ProductionRule = Name : ProductionRuleRHS ;")],
   // State 63:|
    [Error, Error, Shift(42, 2), Error, Error, Shift(41, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 64:{
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Shift(45, 11), Shift(46, 12), Shift(47, 13), Shift(48, 14), Shift(49, 15), Shift(50, 16), Shift(51, 17), Shift(52, 18), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 65:Assignment
    [Error, Error, Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Error, Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Error, Error, Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Error, Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Reduce(55, 2, 19, "55: Assignments = Assignments Assignment"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 66:OptRepeatOperator
    [Error, Error, Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Reduce(60, 2, 23, "60: GrammarSymbolReference = GrammarSymbol OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 67:RepeatOperator
    [Error, Error, Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Error, Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Error, Error, Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Error, Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Reduce(62, 1, 24, "62: OptRepeatOperator = RepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 68:*
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 69:*!
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 70:+
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 71:+!
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 72:?
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 73:?!
    [Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Reduce(71, 0, 26, "71: OptionalRepeatModifiersExpression = EMPTY"), Error, Error, Error, Error, Error, Error, Shift(95, 35), Error, Error, Error, Error, Error, Error, Error, Error],
   // State 74:OptRepeatOperator
    [Error, Error, Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Reduce(61, 2, 23, "61: GrammarSymbolReference = ProductionGroup OptRepeatOperator"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 75:ProductionRuleRHS
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(63, 10), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(101, 28), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 76:?=
    [Error, Error, Shift(42, 2), Error, Error, Shift(103, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 77:=
    [Error, Error, Shift(42, 2), Error, Error, Shift(103, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 78:}
    [Error, Error, Error, Error, Error, Error, Error, Shift(105, 7), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 79:,
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Shift(45, 11), Shift(46, 12), Shift(47, 13), Shift(48, 14), Shift(49, 15), Shift(50, 16), Shift(51, 17), Shift(52, 18), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 80::
    [Error, Error, Shift(111, 2), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(108, 18), Error, Error, Error, Error, Shift(109, 23), Shift(110, 24), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 81:;
    [Error, Error, Error, Error, Error, Reduce(24, 4, 11, "24: TerminalRule = Name : Recognizer ;"), Reduce(24, 4, 11, "24: TerminalRule = Name : Recognizer ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(24, 4, 11, "24: TerminalRule = Name : Recognizer ;")],
   // State 82:{
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(88, 15), Error, Error, Shift(89, 18), Error, Shift(85, 20), Shift(86, 21), Shift(87, 22), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 83:TerminalMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(113, 9), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(114, 19), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 84:TerminalMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(46, 1, 15, "46: TerminalMetaDatas = TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(46, 1, 15, "46: TerminalMetaDatas = TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 85:prefer
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(39, 1, 14, "39: TerminalMetaData = prefer"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(39, 1, 14, "39: TerminalMetaData = prefer"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 86:finish
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(40, 1, 14, "40: TerminalMetaData = finish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(40, 1, 14, "40: TerminalMetaData = finish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 87:nofinish
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(41, 1, 14, "41: TerminalMetaData = nofinish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(41, 1, 14, "41: TerminalMetaData = nofinish"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 88:dynamic
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(42, 1, 14, "42: TerminalMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(42, 1, 14, "42: TerminalMetaData = dynamic"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 89:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(43, 1, 14, "43: TerminalMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(43, 1, 14, "43: TerminalMetaData = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 90:UserMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(44, 1, 14, "44: TerminalMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(44, 1, 14, "44: TerminalMetaData = UserMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 91:;
    [Error, Reduce(9, 5, 3, "9: Import = import StrConst as Name ;"), Error, Error, Error, Reduce(9, 5, 3, "9: Import = import StrConst as Name ;"), Reduce(9, 5, 3, "9: Import = import StrConst as Name ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 92:Production
    [Error, Error, Error, Reduce(16, 3, 7, "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Reduce(16, 3, 7, "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(16, 3, 7, "16: ProductionRuleRHS = ProductionRuleRHS | Production"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 93:ProductionMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(115, 9), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(79, 19), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 94:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Reduce(64, 2, 25, "64: RepeatOperator = * OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 95:[
    [Error, Error, Error, Error, Error, Shift(118, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 96:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Reduce(65, 2, 25, "65: RepeatOperator = *! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 97:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Reduce(66, 2, 25, "66: RepeatOperator = + OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 98:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Reduce(67, 2, 25, "67: RepeatOperator = +! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 99:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Reduce(68, 2, 25, "68: RepeatOperator = ? OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 100:OptionalRepeatModifiersExpression
    [Error, Error, Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Reduce(69, 2, 25, "69: RepeatOperator = ?! OptionalRepeatModifiersExpression"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 101:)
    [Error, Error, Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Reduce(59, 3, 22, "59: ProductionGroup = ( ProductionRuleRHS )"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 102:GrammarSymbolReference
    [Error, Error, Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Reduce(58, 3, 21, "58: BoolAssignment = Name ?= GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 103:Name
    [Error, Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Reduce(75, 1, 29, "75: GrammarSymbol = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 104:GrammarSymbolReference
    [Error, Error, Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Reduce(57, 3, 20, "57: PlainAssignment = Name = GrammarSymbolReference"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 105::
    [Error, Error, Shift(42, 2), Error, Error, Shift(41, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(40, 27), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 106:ProductionMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(37, 3, 13, "37: ProductionMetaDatas = ProductionMetaDatas , ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(37, 3, 13, "37: ProductionMetaDatas = ProductionMetaDatas , ProductionMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 107:Const
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(47, 3, 16, "47: UserMetaData = Name : Const"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(47, 3, 16, "47: UserMetaData = Name : Const"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 108:IntConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(48, 1, 17, "48: Const = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(48, 1, 17, "48: Const = IntConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 109:FloatConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(49, 1, 17, "49: Const = FloatConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(49, 1, 17, "49: Const = FloatConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 110:BoolConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(50, 1, 17, "50: Const = BoolConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(50, 1, 17, "50: Const = BoolConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 111:StrConst
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(51, 1, 17, "51: Const = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(51, 1, 17, "51: Const = StrConst"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 112:TerminalMetaDatas
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(120, 9), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(114, 19), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 113:}
    [Error, Error, Error, Shift(121, 3), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 114:,
    [Error, Error, Error, Error, Error, Shift(54, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(88, 15), Error, Error, Shift(89, 18), Error, Shift(85, 20), Shift(86, 21), Shift(87, 22), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 115:}
    [Error, Error, Error, Reduce(19, 4, 8, "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Reduce(19, 4, 8, "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(19, 4, 8, "19: Production = Assignments { ProductionMetaDatas }"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 116:OptionalRepeatModifiers
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(124, 19), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Shift(123, 36), Error, Error, Error, Error, Error, Error, Error],
   // State 117:OptionalRepeatModifier
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(73, 1, 27, "73: OptionalRepeatModifiers = OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(73, 1, 27, "73: OptionalRepeatModifiers = OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error],
   // State 118:Name
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(74, 1, 28, "74: OptionalRepeatModifier = Name"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(74, 1, 28, "74: OptionalRepeatModifier = Name"), Error, Error, Error, Error, Error, Error, Error],
   // State 119:ProductionRuleRHS
    [Error, Error, Error, Shift(125, 3), Error, Error, Error, Error, Error, Error, Shift(63, 10), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 120:}
    [Error, Error, Error, Shift(126, 3), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 121:;
    [Error, Error, Error, Error, Error, Reduce(27, 6, 11, "27: TerminalRule = Name : { TerminalMetaDatas } ;"), Reduce(27, 6, 11, "27: TerminalRule = Name : { TerminalMetaDatas } ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(27, 6, 11, "27: TerminalRule = Name : { TerminalMetaDatas } ;")],
   // State 122:TerminalMetaData
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(45, 3, 15, "45: TerminalMetaDatas = TerminalMetaDatas , TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(45, 3, 15, "45: TerminalMetaDatas = TerminalMetaDatas , TerminalMetaData"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 123:]
    [Error, Error, Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Reduce(70, 3, 26, "70: OptionalRepeatModifiersExpression = [ OptionalRepeatModifiers ]"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 124:,
    [Error, Error, Error, Error, Error, Shift(118, 5), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error],
   // State 125:;
    [Reduce(15, 7, 6, "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Error, Error, Error, Error, Reduce(15, 7, 6, "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Reduce(15, 7, 6, "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(15, 7, 6, "15: ProductionRule = Name { ProductionMetaDatas } : ProductionRuleRHS ;")],
   // State 126:;
    [Error, Error, Error, Error, Error, Reduce(26, 7, 11, "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;"), Reduce(26, 7, 11, "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(26, 7, 11, "26: TerminalRule = Name : Recognizer { TerminalMetaDatas } ;")],
   // State 127:OptionalRepeatModifier
    [Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(72, 3, 27, "72: OptionalRepeatModifiers = OptionalRepeatModifiers , OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Reduce(72, 3, 27, "72: OptionalRepeatModifiers = OptionalRepeatModifiers , OptionalRepeatModifier"), Error, Error, Error, Error, Error, Error, Error]   
],

   gotos: [
   // State 0:S'
    [None, Some(1), Some(3), Some(5), Some(2), Some(7), Some(9), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 1:PGFile
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 2:ProductionRules
    [None, None, None, None, None, Some(12), Some(9), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 3:Imports
    [None, None, None, Some(14), Some(13), Some(7), Some(9), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 4:terminals
    [None, None, None, None, None, None, None, None, None, Some(15), Some(16), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 5:Import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 6:import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 7:ProductionRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 8:Action
    [None, None, None, None, None, None, Some(21), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 9:ProductionRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 10:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 11:terminals
    [None, None, None, None, None, None, None, None, None, Some(24), Some(16), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 12:ProductionRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 13:ProductionRules
    [None, None, None, None, None, Some(12), Some(9), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 14:Import
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 15:TerminalRules
    [None, None, None, None, None, None, None, None, None, None, Some(26), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 16:TerminalRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 17:Action
    [None, None, None, None, None, None, None, None, None, None, None, Some(27), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 18:TerminalRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 19:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 20:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 21:ProductionRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 22::
    [None, None, None, None, None, None, None, Some(31), Some(32), None, None, None, None, None, None, None, None, None, Some(34), Some(33), Some(35), Some(36), Some(39), Some(37), None, None, None, None, None, Some(38), None, None, None, None, None, None],
   // State 23:{
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(44), Some(43), None, None, Some(53), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 24:TerminalRules
    [None, None, None, None, None, None, None, None, None, None, Some(26), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 25:terminals
    [None, None, None, None, None, None, None, None, None, Some(55), Some(16), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 26:TerminalRuleWithAction
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 27:TerminalRule
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 28::
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(56), None, None, None, None, None],
   // State 29:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 30:as
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 31:ProductionRuleRHS
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 32:Production
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 33:Assignments
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(65), None, Some(35), Some(36), Some(39), Some(37), None, None, None, None, None, Some(38), None, None, None, None, None, None],
   // State 34:Assignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 35:PlainAssignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 36:BoolAssignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 37:GrammarSymbolReference
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 38:GrammarSymbol
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(66), Some(67), None, None, None, None, None, None, None, None, None, None],
   // State 39:ProductionGroup
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(74), Some(67), None, None, None, None, None, None, None, None, None, None],
   // State 40:(
    [None, None, None, None, None, None, None, Some(75), Some(32), None, None, None, None, None, None, None, None, None, Some(34), Some(33), Some(35), Some(36), Some(39), Some(37), None, None, None, None, None, Some(38), None, None, None, None, None, None],
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
    [None, None, None, None, None, None, None, None, None, None, Some(26), Some(18), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 56:Recognizer
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 57:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 58:{
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(84), Some(83), Some(90), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 59:StrConst
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 60:RegExTerm
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 61:Name
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 62:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 63:|
    [None, None, None, None, None, None, None, None, Some(92), None, None, None, None, None, None, None, None, None, Some(34), Some(33), Some(35), Some(36), Some(39), Some(37), None, None, None, None, None, Some(38), None, None, None, None, None, None],
   // State 64:{
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(44), Some(93), None, None, Some(53), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 65:Assignment
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 66:OptRepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 67:RepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 68:*
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(94), None, None, None, None, None, None, None, None, None],
   // State 69:*!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(96), None, None, None, None, None, None, None, None, None],
   // State 70:+
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(97), None, None, None, None, None, None, None, None, None],
   // State 71:+!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(98), None, None, None, None, None, None, None, None, None],
   // State 72:?
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(99), None, None, None, None, None, None, None, None, None],
   // State 73:?!
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(100), None, None, None, None, None, None, None, None, None],
   // State 74:OptRepeatOperator
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 75:ProductionRuleRHS
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 76:?=
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(39), Some(102), None, None, None, None, None, Some(38), None, None, None, None, None, None],
   // State 77:=
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(39), Some(104), None, None, None, None, None, Some(38), None, None, None, None, None, None],
   // State 78:}
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 79:,
    [None, None, None, None, None, None, None, None, None, None, None, None, Some(106), None, None, None, Some(53), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 80::
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(107), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 81:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 82:{
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(84), Some(112), Some(90), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
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
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(116), Some(117), None, None, None, None, None, None, None],
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
    [None, None, None, None, None, None, None, Some(119), Some(32), None, None, None, None, None, None, None, None, None, Some(34), Some(33), Some(35), Some(36), Some(39), Some(37), None, None, None, None, None, Some(38), None, None, None, None, None, None],
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
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(122), None, Some(90), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
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
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(127), None, None, None, None, None, None, None],
   // State 125:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 126:;
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
   // State 127:OptionalRepeatModifier
    [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]   
]

};

impl ParserDefinition for RustemoParserDefinition {
    fn action(&self, state_index: usize, term_index: usize) -> Action {
        PARSER_DEFINITION.actions[state_index][term_index]
    }
    fn goto(&self, state_index: usize, nonterm_id: usize) -> usize {
        PARSER_DEFINITION.gotos[state_index][nonterm_id].unwrap()
    }
}

pub struct RustemoLexerDefinition {
    terminals: TerminalInfos<TERMINAL_NO>,
    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
}

pub(in crate::lang) static LEXER_DEFINITION: RustemoLexerDefinition = RustemoLexerDefinition {
   terminals: [
    TerminalInfo{
       id: 0,
       name: "terminals",
       location: None,
   },
    TerminalInfo{
       id: 1,
       name: "import",
       location: None,
   },
    TerminalInfo{
       id: 2,
       name: "StrConst",
       location: None,
   },
    TerminalInfo{
       id: 3,
       name: ";",
       location: None,
   },
    TerminalInfo{
       id: 4,
       name: "as",
       location: None,
   },
    TerminalInfo{
       id: 5,
       name: "Name",
       location: None,
   },
    TerminalInfo{
       id: 6,
       name: "Action",
       location: None,
   },
    TerminalInfo{
       id: 7,
       name: ":",
       location: None,
   },
    TerminalInfo{
       id: 8,
       name: "{",
       location: None,
   },
    TerminalInfo{
       id: 9,
       name: "}",
       location: None,
   },
    TerminalInfo{
       id: 10,
       name: "|",
       location: None,
   },
    TerminalInfo{
       id: 11,
       name: "left",
       location: None,
   },
    TerminalInfo{
       id: 12,
       name: "reduce",
       location: None,
   },
    TerminalInfo{
       id: 13,
       name: "right",
       location: None,
   },
    TerminalInfo{
       id: 14,
       name: "shift",
       location: None,
   },
    TerminalInfo{
       id: 15,
       name: "dynamic",
       location: None,
   },
    TerminalInfo{
       id: 16,
       name: "nops",
       location: None,
   },
    TerminalInfo{
       id: 17,
       name: "nopse",
       location: None,
   },
    TerminalInfo{
       id: 18,
       name: "IntConst",
       location: None,
   },
    TerminalInfo{
       id: 19,
       name: ",",
       location: None,
   },
    TerminalInfo{
       id: 20,
       name: "prefer",
       location: None,
   },
    TerminalInfo{
       id: 21,
       name: "finish",
       location: None,
   },
    TerminalInfo{
       id: 22,
       name: "nofinish",
       location: None,
   },
    TerminalInfo{
       id: 23,
       name: "FloatConst",
       location: None,
   },
    TerminalInfo{
       id: 24,
       name: "BoolConst",
       location: None,
   },
    TerminalInfo{
       id: 25,
       name: "=",
       location: None,
   },
    TerminalInfo{
       id: 26,
       name: "?=",
       location: None,
   },
    TerminalInfo{
       id: 27,
       name: "(",
       location: None,
   },
    TerminalInfo{
       id: 28,
       name: ")",
       location: None,
   },
    TerminalInfo{
       id: 29,
       name: "*",
       location: None,
   },
    TerminalInfo{
       id: 30,
       name: "*!",
       location: None,
   },
    TerminalInfo{
       id: 31,
       name: "+",
       location: None,
   },
    TerminalInfo{
       id: 32,
       name: "+!",
       location: None,
   },
    TerminalInfo{
       id: 33,
       name: "?",
       location: None,
   },
    TerminalInfo{
       id: 34,
       name: "?!",
       location: None,
   },
    TerminalInfo{
       id: 35,
       name: "[",
       location: None,
   },
    TerminalInfo{
       id: 36,
       name: "]",
       location: None,
   },
    TerminalInfo{
       id: 37,
       name: "RegExTerm",
       location: None,
   },
    TerminalInfo{
       id: 38,
       name: "WS",
       location: None,
   },
    TerminalInfo{
       id: 39,
       name: "/*",
       location: None,
   },
    TerminalInfo{
       id: 40,
       name: "*/",
       location: None,
   },
    TerminalInfo{
       id: 41,
       name: "CommentLine",
       location: None,
   },
    TerminalInfo{
       id: 42,
       name: "NotComment",
       location: None,
   },
    TerminalInfo{
       id: 43,
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

    fn recognizers(&self, state_index: usize) -> RecognizerIterator<Self::Recognizer> {
            RecognizerIterator {
                terminals: &LEXER_DEFINITION.terminals,
                terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index][..],
                recognizers: &LEXER_DEFINITION.recognizers,
                index: 0
            }
    }
}

pub struct RustemoBuilder<'i, I: 'i> {
    res_stack: Vec<Symbol>,
    phantom: PhantomData<&'i I>
}

impl<'i, I> Builder for RustemoBuilder<'i, I>
{
    type Output = Symbol;
    type Lexer = GrammarLexer<'i>;

    fn new() -> Self {
        RustemoBuilder {
            res_stack: vec![],
            phantom: PhantomData,
        }
    }

    fn shift_action(&mut self, term_kind: usize, token: Token<<Self::Lexer as Lexer>::Input>) {
        let termval = match TermKind::try_from(term_kind).unwrap() {
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

    fn reduce_action(&mut self, prod_kind: usize, prod_len: usize, _prod_str: &'static str) {
        let prod = match ProdKind::try_from(prod_kind).unwrap() {
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

    fn get_result(&mut self) -> Self::Output {
       self.res_stack.pop().unwrap()
    }
}

