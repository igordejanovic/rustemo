"""
Bootstrap rustemo grammar parser using parglare.
"""
import os
from datetime import datetime
from parglare.tables import SHIFT, REDUCE, ACCEPT
from parglare import Parser, Grammar, visitor
from parglare.grammar import pg_productions, PGFILE, EMPTY, get_grammar_parser
from parglare.actions import pass_none
from parglare.grammar import DEFAULT_PRIORITY


non_terminals = {}
terminals = {}
productions = {}

def cap(name):
    return name[0].upper() + name[1:]

def camel_case(name):
    return ''.join([cap(n) for n in name.split('_')])

def terminal_name(name):
    name = {
        ';': 'SemiColon',
        ':': 'Colon',
        '{': 'OBrace',
        '}': 'CBrace',
        '|': 'Bar',
        ',': 'Comma',
        '=': 'Equals',
        '?=': 'QEquals',
        '(': 'OBracket',
        ')': 'CBracket',
        '*': 'Asterisk',
        '*!': 'AsteriskGready',
        '+': 'Plus',
        '+!': 'PlusGready',
        '?': 'Question',
        '?!': 'QuestionGready',
        '[': 'OSquare',
        ']': 'CSquare',
        '/*': 'OComment',
        '*/': 'CComment'
    }.get(name, name)

    return camel_case(name)


def snake_case(name):
    """
    Convert CamelCase to snake_case.
    """
    # If a letter is uppercase and is preceded by lowercase underscore is added.
    lowercase = False
    result = ''
    for l in name:
        if l.isupper():
            if lowercase:
                result += '_'
        result += l.lower()
        lowercase = l.islower()
    return result


def create_symbol_ids(grammar):
    for idx, terminal in enumerate(grammar.terminals):
        terminals[terminal] = idx
    for idx, nonterminal in enumerate(grammar.nonterminals):
        non_terminals[nonterminal] = idx
    for prod in grammar.productions:
        if prod.symbol.name == 'S\'':
            continue
        productions[prod.prod_id] = f'{prod.symbol.name}P{prod.prod_symbol_id}'


def generate_terminals(grammar):
    out = []
    for idx, terminal in enumerate(grammar.terminals.values()):
        out.append('    TerminalInfo{')
        out.append(f'       id: TermIndex({idx}),')
        out.append(f'       name: "{terminal.name}",')
        #out.append(f'       fqn: "{terminal.fqn}",')
        out.append('       location: None,')
        out.append('   },')
    return '\n'.join(out)


def generate_recognizers(grammar):
    """In each state we have to find out what token is ahead. Thus, we need a table
    of recognizers indexed by terminal id.

    """

    out = []
    for idx, terminal in enumerate(grammar.terminals.values()):
        out.append(f'       // {idx}:{terminal.name}')
        out.append('       |input: &str| {')
        out.append(f'           logn!("Recognizing <{terminal.name.replace("}", "}}").replace("{", "{{")}> -- ");')
        if type(terminal._recognizer).__name__ == 'StringRecognizer':
            out.append(f'           if input.starts_with("{terminal._recognizer.value}"){{')
            out.append(f'              log!("recognized");')
            out.append(f'              Some("{terminal._recognizer.value}")')
            out.append('           } else {')
            out.append(f'              log!("not recognized");')
            out.append('              None')
            out.append('           }')
        elif type(terminal._recognizer).__name__ == 'RegExRecognizer':
            out.append(f'           let regex = Regex::new(r#"^({terminal._recognizer._regex})"#).unwrap();'.replace(r'\/', r'/'))
            out.append('           let match_str = regex.find(input);')
            out.append('           match match_str {')
            out.append('               Some(x) => {')
            out.append('                   let x_str = x.as_str();')
            out.append('                   log!("recognized <{}>", x_str);')
            out.append('                   Some(x_str)')
            out.append('               },')
            out.append('               None => {')
            out.append('                   log!("not recognized");')
            out.append('                   None')
            out.append('               }')
            out.append('           }')
        else:
            out.append('           if input.is_empty() { Some("") } else { None }')
        out.append('       },')
    return '\n'.join(out)


def max_actions(table):
    actions = []
    for state in table.states:
        actions.append(len(state.actions))
    return max(actions)


def generate_recognizers_for_state(table):
    out = []
    for state in table.states:
        action_terms = []
        for action_term in state.actions:
            action_terms.append(terminals[action_term.name])
        out.append(action_terms)
    max_actions = max(len(a) for a in out)
    for a in out:
        while len(a) < max_actions:
            a.append(None)

    def action_str(actions):
        return '    [{}]'.format(', '.join(f'Some({a})' if a is not None else 'None'
                             for a in actions))

    f.write(f'  terminals_for_state: [\n')
    return ',\n'.join([f'   // State {idx}:{table.states[idx].symbol}\n{action_str(o)}'
                       for idx, o in enumerate(out)])


class Action:
    def __init__(self, action):
        self.action = action.action
        if action.prod:
            self.param = 'ProdIndex({}), {}, NonTermIndex({}), "{}"'.format(
                action.prod.prod_id,
                len(action.prod.rhs),
                non_terminals[action.prod.symbol.name],
                action.prod)
        elif action.state:
            self.param = 'StateIndex({}), TermIndex({})'.format(
                action.state.state_id,
                terminals[action.state.symbol.name])
        else:
            self.param = None

    def __repr__(self):
        if self.action == SHIFT:
            return f'Shift({self.param})'
        elif self.action == REDUCE:
            return f'Reduce({self.param})'
        elif self.action == ACCEPT:
            return 'Accept'
        else:
            return 'Error'


def generate_actions_table(table):
    out = []
    for state in table.states:
        state_actions = [None] * no_terminals
        for term, actions in state.actions.items():
            term_idx = terminals[term.name]
            # There can be only one action for LR parser
            state_actions[term_idx] = Action(actions[0])
        state_actions = ', '.join([str(a) if a is not None else 'Error'
                                   for a in state_actions])
        out.append('    [{}]'.format(state_actions))

    return ',\n'.join([f'   // State {idx}:{table.states[idx].symbol}\n{o}'
                       for idx, o in enumerate(out)])

def generate_gotos_table(table):
    out = []
    for state in table.states:
        state_gotos = [None] * no_nonterminals
        for nt, goto_state in state.gotos.items():
            nt_idx = non_terminals[nt.name]
            state_gotos[nt_idx] = goto_state.state_id
        state_gotos = ', '.join(['Some(StateIndex({}))'.format(g) if g is not None else 'None'
                                 for g in state_gotos])
        out.append('    [{}]'.format(state_gotos))

    return ',\n'.join([f'   // State {idx}:{table.states[idx].symbol}\n{o}'
                       for idx, o in enumerate(out)])

def generate_symbol_names(grammar):
    """We need to know (non)terminal names (and possible other meta-data) for error
    reporting. If a dynamic structure is need (heap allocated) that could be
    postponed to the point when error occurs (or perhaps a dumb switch case code
    should be generated that provides the same functionallity but statically).

    """

if __name__ == '__main__':
    parser = get_grammar_parser(False, False)
    del parser.grammar.terminals['EMPTY']
    create_symbol_ids(parser.grammar)
    no_terminals = len(terminals)
    no_nonterminals = len(non_terminals)
    no_states = len(parser.table.states)
    with open('rustemo.rs', 'w') as f:
        f.write('// Generated on {} from bootstrap.py. Do not edit!\n\n'
                .format(datetime.now()))

        f.write('use regex::Regex;\n')
        f.write('use std::convert::TryFrom;\n\n');
        f.write('use std::marker::PhantomData;\n')
        f.write('use rustemort::lexer::{Lexer, DefaultLexer, Token, LexerDefinition, RecognizerIterator};\n')
        f.write('use rustemort::lr::{LRParser, LRContext, ParserDefinition};\n')
        f.write('use rustemort::lr::Action::{self, Shift, Reduce, Accept, Error};\n')
        f.write('use rustemort::parser::Parser;\n')
        f.write('use rustemort::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};\n')
        f.write('use rustemort::builder::Builder;\n')
        f.write('use rustemort::grammar::{TerminalInfo, TerminalInfos, TerminalsState};\n')
        f.write('use rustemort::debug::{log, logn};\n')
        f.write('use super::parser::RustemoLexer;\n')
        f.write('use super::rustemo_types::{TermKind, ProdKind, Terminal, NonTerminal, Symbol};\n\n')
        f.write('use super::types::*;\n\n')

        f.write(f'const TERMINAL_NO: usize = {no_terminals};\n')
        f.write(f'const NONTERMINAL_NO: usize = {no_nonterminals};\n')
        f.write(f'const STATE_NO: usize = {no_states};\n')
        f.write(f'const MAX_ACTIONS: usize = {max_actions(parser.table)};\n\n')

        f.write('pub struct RustemoParserDefinition {\n')
        f.write('    actions: [[Action; TERMINAL_NO]; STATE_NO],\n')
        f.write('    gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]\n')
        f.write('}\n\n')

        f.write('pub(in crate) static PARSER_DEFINITION: RustemoParserDefinition = RustemoParserDefinition {\n')
        f.write('   actions: [\n')
        f.write(generate_actions_table(parser.table))
        f.write('   \n],\n\n')

        f.write('   gotos: [\n')
        f.write(generate_gotos_table(parser.table))
        f.write('   \n]\n\n')
        f.write('};\n\n')

        f.write('impl ParserDefinition for RustemoParserDefinition {\n')
        f.write('    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {\n')
        f.write('        PARSER_DEFINITION.actions[state_index.0][term_index.0]\n')
        f.write('    }\n')
        f.write('    fn goto(&self, state_index: StateIndex, nonterm_id: NonTermIndex) -> StateIndex {\n')
        f.write('        PARSER_DEFINITION.gotos[state_index.0][nonterm_id.0].unwrap()\n')
        f.write('    }\n')
        f.write('}\n\n')

        f.write('pub struct RustemoParser<\'i>(LRParser<&\'i str, RustemoParserDefinition>);\n\n');

        f.write('impl<\'i> Default for RustemoParser<\'i> {\n')
        f.write('    fn default() -> Self {\n')
        f.write('        Self(LRParser {\n')
        f.write('            context: LRContext {\n')
        f.write('                parse_stack: vec![StateIndex(0)],\n')
        f.write('                current_state: StateIndex(0),\n')
        f.write('                position: 0,\n')
        f.write('                token: None,\n')
        f.write('            },\n')
        f.write('            definition: &PARSER_DEFINITION,\n')
        f.write('        })\n')
        f.write('    }\n')
        f.write('}\n\n')

        f.write('pub struct RustemoLexerDefinition {\n')
        f.write('    terminals: TerminalInfos<TERMINAL_NO>,\n')
        f.write('    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,\n')
        f.write('    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]\n')
        f.write('}\n\n')

        f.write('pub(in crate) static LEXER_DEFINITION: RustemoLexerDefinition = RustemoLexerDefinition {\n')
        f.write('   terminals: [\n')
        f.write(generate_terminals(parser.grammar))
        f.write('   \n],\n\n')
        f.write('   // Expected terminals/tokens indexed by state id.\n')
        f.write('   // Sorted by priority.\n')
        f.write(generate_recognizers_for_state(parser.table))
        f.write('   \n],\n\n')
        f.write('   recognizers: [\n')
        f.write(generate_recognizers(parser.grammar))
        f.write('\n')
        f.write('   ]\n')
        f.write('};\n\n')

        f.write('pub struct RustemoLexer<\'i>(DefaultLexer<\'i, RustemoLexerDefinition>);\n\n');

        f.write('impl<\'i> Lexer for RustemoLexer<\'i> {\n')
        f.write('    type Input = &\'i str;\n')
        f.write('\n')
        f.write('    fn next_token(\n')
        f.write('        &self,\n')
        f.write('        context: &mut impl rustemort::parser::Context<Self::Input>,\n')
        f.write('    ) -> Option<rustemort::lexer::Token<Self::Input>> {\n')
        f.write('        self.0.next_token(context)\n')
        f.write('    }\n')
        f.write('}\n')
        f.write('\n')
        f.write('// Enables creating a lexer from a reference to an object that can be converted\n')
        f.write('// to a string reference.\n')
        f.write('impl<\'i, T> From<&\'i T> for RustemoLexer<\'i>\n')
        f.write('where\n')
        f.write('    T: AsRef<str> + ?Sized,\n')
        f.write('{\n')
        f.write('    fn from(input: &\'i T) -> Self {\n')
        f.write('        Self(DefaultLexer::new(input.as_ref(), &LEXER_DEFINITION))\n')
        f.write('    }\n')
        f.write('}\n\n')

        f.write('impl LexerDefinition for RustemoLexerDefinition {\n')
        f.write('    type Recognizer = for<\'i> fn(&\'i str) -> Option<&\'i str>;\n\n')
        f.write('    fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {\n')
        f.write('            RecognizerIterator {\n')
        f.write('                terminals: &LEXER_DEFINITION.terminals,\n')
        f.write('                terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],\n')
        f.write('                recognizers: &LEXER_DEFINITION.recognizers,\n')
        f.write('                index: 0\n')
        f.write('            }\n')
        f.write('    }\n')
        f.write('}\n\n')

        f.write('pub struct RustemoBuilder<\'i, I: \'i> {\n')
        f.write('    res_stack: Vec<Symbol>,\n')
        f.write('    phantom: PhantomData<&\'i I>\n')
        f.write('}\n')
        f.write('\n')
        f.write('impl<\'i, I> Builder for RustemoBuilder<\'i, I>\n')
        f.write('{\n')
        f.write('    type Output = Symbol;\n')
        f.write('    type Lexer = RustemoLexer<\'i>;\n')
        f.write('\n')
        f.write('    fn new() -> Self {\n')
        f.write('        RustemoBuilder {\n')
        f.write('            res_stack: vec![],\n')
        f.write('            phantom: PhantomData,\n')
        f.write('        }\n')
        f.write('    }\n')
        f.write('\n')
        f.write('    fn shift_action(&mut self, term_kind: TermIndex, token: Token<<Self::Lexer as Lexer>::Input>) {\n')
        f.write('        let termval = match TermKind::try_from(term_kind.0).unwrap() {\n')
        for t_name, idx in terminals.items():
            term = parser.grammar.terminals[t_name]
            t_name = terminal_name(t_name)
            action_call = ''
            if not hasattr(term.recognizer, 'value'):
                action_name = snake_case(t_name)
                action_call = f'({action_name}(token))'
            f.write(f'            TermKind::{t_name} => Terminal::{t_name}{action_call},\n')
        f.write('        };\n')
        f.write('        self.res_stack.push(Symbol::Terminal(termval));\n')
        f.write('    }\n\n')
        f.write('    fn reduce_action(&mut self, prod_kind: ProdIndex, prod_len: usize, _prod_str: &\'static str) {\n')
        f.write('        let prod = match ProdKind::try_from(prod_kind.0).unwrap() {\n')
        for nt_name, nt in parser.grammar.nonterminals.items():
            if nt_name == 'S\'': continue
            nt_name = camel_case(nt_name)
            for idx, prod in enumerate(nt.productions):
                name = f'{nt_name}P{idx}'
                if len(prod.rhs) >= 1:
                    f.write(f'            ProdKind::{name} => {{\n')
                    f.write(f'                let mut i = self.res_stack.split_off(self.res_stack.len()-{len(prod.rhs)}).into_iter();\n')
                    nexts = ', '.join(['i.next().unwrap()'] * len(prod.rhs))
                    f.write(f'                match ({nexts}) {{\n')
                    params = []
                    call_params = []
                    pidx = 0
                    for p in prod.rhs:
                        if p.__class__.__name__ == 'Terminal' and hasattr(p.recognizer, 'value'):
                            # String recognition
                            params.append('_')
                            continue
                        params.append('Symbol::{t}({t}::{name}(p{pidx}))'
                                      .format(t=p.__class__.__name__, name=camel_case(p.name), pidx=pidx))
                        call_params.append(f'p{pidx}')
                        pidx += 1
                    params = ', '.join(params)
                    call_params = ', '.join(call_params)
                    f.write(f'                    ({params}) => NonTerminal::{nt_name}({snake_case(name)}({call_params})),\n')
                    f.write('                    _ => panic!("Invalid symbol parse stack data.")\n')
                    f.write('                }\n')
                    f.write('            },\n')
                else:
                    f.write(f'            ProdKind::{name} => NonTerminal::{nt_name}({snake_case(name)}()),\n')
        f.write('        };\n')
        f.write('        self.res_stack.push(Symbol::NonTerminal(prod));\n')
        f.write('    }\n\n')
        f.write('    fn get_result(&mut self) -> Self::Output {\n')
        f.write('       self.res_stack.pop().unwrap()\n')
        f.write('    }\n')
        f.write('}\n\n')


    with open('rustemo_types.rs', 'w') as f:
        f.write('// Generated on {} from bootstrap.py. Do not edit!\n\n'
                .format(datetime.now()))
        f.write('use num_enum::TryFromPrimitive;\n')
        f.write('use super::types::*;\n\n')

        f.write('#[derive(Debug, Copy, Clone, TryFromPrimitive)]\n')
        f.write('#[repr(usize)]\n')
        f.write('pub enum TermKind {\n')
        for t_name, idx in terminals.items():
            t_name = terminal_name(t_name)
            f.write(f'\t{t_name} = {idx},\n')
        f.write('}\n\n')

        f.write('#[derive(Debug, Copy, Clone)]\n')
        f.write('pub enum NonTermKind {\n')
        for nt_name, idx in non_terminals.items():
            if nt_name == 'S\'': continue
            f.write(f'\t{nt_name} = {idx},\n')
        f.write('}\n\n')

        f.write('#[derive(Debug)]\n')
        f.write('pub enum Symbol {\n')
        f.write('\tTerminal(Terminal),\n')
        f.write('\tNonTerminal(NonTerminal)\n')
        f.write('}\n\n')

        f.write('#[derive(Debug)]\n')
        f.write('pub enum Terminal {\n')
        for t_name, idx in terminals.items():
            term = parser.grammar.terminals[t_name]
            if hasattr(term.recognizer, 'value'):
                t_name = terminal_name(t_name)
                f.write(f'\t{t_name},\n')
            else:
                t_name = terminal_name(t_name)
                f.write(f'\t{t_name}({t_name}),\n')
        f.write('}\n\n')

        f.write('#[derive(Debug)]\n')
        f.write('pub enum NonTerminal {\n')
        for nt in non_terminals:
            if nt == 'S\'': continue
            nt = camel_case(nt)
            f.write(f'\t{nt}({nt}),\n')
        f.write('\tEmpty\n')
        f.write('}\n\n')

        # TODO: Remove this, leave only prod_kinds
        prod_kinds = []
        for nt_name, nt in parser.grammar.nonterminals.items():
            if nt_name == 'S\'': continue
            nt_name = camel_case(nt_name)
            # f.write(f'pub enum {nt_name} {{\n')
            for idx, prod in enumerate(nt.productions):
                if len(prod.rhs) >= 0:
                    name = f'{nt_name}P{idx}'
                    # f.write(f'\tP{idx}({name}),\n')
                    prod_kinds.append(name)
                else:
                    pass
            #         f.write(f'\tEmpty(Empty),\n')
            # f.write(f'}}\n\n')


        f.write('#[derive(Copy, Clone, TryFromPrimitive)]\n')
        f.write('#[repr(usize)]\n')
        f.write('pub enum ProdKind {\n')
        for idx, prodkind in enumerate(prod_kinds):
            f.write(f'\t{prodkind} = {idx + 1},\n')
        f.write('}\n\n')
