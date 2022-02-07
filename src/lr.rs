use crate::builder::Builder;
use crate::debug::log;
use crate::index::StateIndex;
use crate::lexer::{Lexer, Token};
use crate::parser::{Action, Context, Parser, ParserDefinition};

#[derive(Debug)]
pub struct LRContext {
    pub parse_stack: Vec<StateIndex>,
    pub current_state: StateIndex,
    pub position: usize,
}

impl LRContext {
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

impl Context for LRContext {
    #[inline]
    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.position = position
    }

    #[inline]
    fn state(&self) -> StateIndex {
        self.current_state
    }
}

#[derive(Debug)]
pub struct LRParser<D: ParserDefinition + 'static> {
    pub context: LRContext,
    pub definition: &'static D,
}

impl<D: ParserDefinition> LRParser<D> {
    pub fn new(definition: &'static D) -> Self {
        Self {
            context: LRContext {
                parse_stack: vec![StateIndex(0)],
                current_state: StateIndex(0),
                position: 0,
            },
            definition,
        }
    }

    #[inline]
    fn next_token<L: Lexer>(&mut self, lexer: &mut L) -> Token<L::Input> {
        match lexer.next_token(&mut self.context) {
            Some(t) => t,
            None => {
                panic!("Error at position {}", self.context.position());
            }
        }
    }
}

impl<D, L, B> Parser<L, B> for LRParser<D>
where
    D: ParserDefinition,
    L: Lexer,
    B: Builder<Lexer = L>,
{
    fn parse(&mut self, mut lexer: L) -> B::Output {
        use Action::*;
        let mut builder = B::new();
        let mut next_token = self.next_token(&mut lexer);
        loop {
            let current_state = *self.context.parse_stack.last().unwrap();
            log!("Stack: {:?}", self.context.parse_stack);
            log!("Current state: {:?}", current_state);
            log!("Token ahead: {:?}", next_token);

            let action = self.definition.action(current_state, next_token.index());

            match action {
                Shift(state_id, term_kind) => {
                    log!(
                        "Shifting to state {:?} with token {:?}",
                        state_id,
                        next_token
                    );
                    self.context.to_state(state_id);
                    builder.shift_action(term_kind, next_token);
                    next_token = self.next_token(&mut lexer);
                }
                Reduce(prod_kind, prod_len, nonterm_id, prod_str) => {
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
                    builder.reduce_action(prod_kind, prod_len, prod_str);
                }
                Accept => break,
                Error => panic!("Error!"),
            }
        }
        builder.get_result()
    }
}

mod tests {}
