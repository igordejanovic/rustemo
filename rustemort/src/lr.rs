use core::fmt::Debug;
use crate::builder::Builder;
use crate::debug::log;
use crate::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
use crate::lexer::{Lexer, Token};
use crate::parser::{Context, Parser};

#[derive(Debug)]
pub struct LRContext<I> {
    pub parse_stack: Vec<StateIndex>,
    pub current_state: StateIndex,
    pub position: usize,
    pub token: Option<Token<I>>,
}

impl<I> LRContext<I> {
    #[inline]
    fn to_state(&mut self, state: StateIndex) {
        self.parse_stack.push(state);
        self.current_state = state;
    }
    #[inline]
    fn pop_states(&mut self, states: usize) -> StateIndex {
        let _ = self.parse_stack.split_off(self.parse_stack.len() - states);
        self.current_state = *self.parse_stack.last().unwrap();
        self.current_state
    }
}

impl<I> Context<I> for LRContext<I> {
    #[inline]
    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.position = position
    }

    fn token_ahead(&self) -> &Option<Token<I>> {
        &self.token
    }

    fn set_token_ahead(&mut self, token: Option<Token<I>>) {
        self.token = token;
    }

    #[inline]
    fn state(&self) -> StateIndex {
        self.current_state
    }
}

pub trait ParserDefinition {
    fn action(&self, state: StateIndex, term_index: TermIndex) -> Action;
    fn goto(&self, state: StateIndex, nonterm_id: NonTermIndex) -> StateIndex;
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Shift(StateIndex, TermIndex),
    Reduce(ProdIndex, usize, NonTermIndex, &'static str),
    Accept,
    Error,
}

#[derive(Debug)]
pub struct LRParser<I, D: ParserDefinition + 'static> {
    pub context: LRContext<I>,
    pub definition: &'static D,
}

impl<I: Debug, D: ParserDefinition> LRParser<I, D> {
    pub fn new(definition: &'static D) -> Self {
        Self {
            context: LRContext {
                parse_stack: vec![StateIndex(0)],
                current_state: StateIndex(0),
                position: 0,
                token: None,
            },
            definition,
        }
    }

    #[inline]
    fn next_token<L: Lexer<Input=I>>(&mut self, lexer: &L) -> Token<L::Input> {
        match lexer.next_token(&mut self.context) {
            Some(t) => t,
            None => {
                panic!("Error at position {}", self.context.position());
            }
        }
    }
}

impl<D, L, B, I> Parser<L, B> for LRParser<I, D>
where
    D: ParserDefinition,
    I: Debug,
    L: Lexer<Input=I>,
    B: Builder<Lexer = L>,
{
    fn parse(&mut self, lexer: L, mut builder: B) -> B::Output {
        use Action::*;
        let mut next_token = self.next_token(&lexer);
        loop {
            let current_state = *self.context.parse_stack.last().unwrap();
            log!("Stack: {:?}", self.context.parse_stack);
            log!("Current state: {:?}", current_state);
            log!("Token ahead: {:?}", next_token);

            let action = self.definition.action(current_state, next_token.index());

            match action {
                Shift(state_id, term_idx) => {
                    log!(
                        "Shifting to state {:?} with token {:?}",
                        state_id,
                        next_token
                    );
                    self.context.to_state(state_id);
                    builder.shift_action(term_idx, next_token);
                    next_token = self.next_token(&lexer);
                }
                Reduce(prod_idx, prod_len, nonterm_id, prod_str) => {
                    log!(
                        "Reduce by production '{:?}', size {:?}, non-terminal {:?}",
                        prod_str,
                        prod_len,
                        nonterm_id
                    );
                    let from_state = self.context.pop_states(prod_len);
                    let to_state = self.definition.goto(from_state, nonterm_id);
                    self.context.to_state(to_state);
                    log!("GOTO {:?} -> {:?}", from_state, to_state);
                    builder.reduce_action(prod_idx, prod_len, prod_str);
                }
                Accept => break,
                Error => panic!("Error!"),
            }
        }
        builder.get_result()
    }
}

mod tests {}
