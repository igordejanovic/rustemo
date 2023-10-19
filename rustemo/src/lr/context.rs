use std::ops::Range;

use crate::{
    context::Context, input::Input, lexer::Token, location::Location,
    parser::State,
};

/// [`Context`] implementation for LR parsing
#[derive(Debug)]
pub struct LRContext<'i, I: Input + ?Sized, S, TK> {
    position: usize,

    /// The range of token/non-terminal during shift/reduce operation.
    range: Range<usize>,

    /// Similar to position but has line/column format for text based inputs.
    ///
    /// If this prove to be pricey overhead we might make tracking of this info
    /// configurable.
    location: Location,

    /// Layout before the lookahead token (e.g. whitespaces, comments...)
    layout_ahead: Option<&'i I>,

    token_ahead: Option<Token<'i, I, TK>>,

    state: S,
}

impl<'i, I: Input + ?Sized, S: Default, TK> Default
    for LRContext<'i, I, S, TK>
{
    fn default() -> Self {
        Self::new(0)
    }
}

impl<'i, I: Input + ?Sized, S: Default, TK> LRContext<'i, I, S, TK> {
    pub fn new(position: usize) -> Self {
        Self {
            position,
            location: I::start_location(),
            layout_ahead: None,
            range: 0..0,
            token_ahead: None,
            state: S::default(),
        }
    }
}

impl<'i, I, S, TK> Context<'i, I, S, TK> for LRContext<'i, I, S, TK>
where
    I: Input + ?Sized,
    S: State,
{
    #[inline]
    fn state(&self) -> S {
        self.state
    }

    #[inline]
    fn set_state(&mut self, state: S) {
        self.state = state
    }

    #[inline]
    fn position(&self) -> usize {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.position = position
    }

    #[inline]
    fn location(&self) -> Location {
        self.location
    }

    #[inline]
    fn set_location(&mut self, location: Location) {
        self.location = location
    }

    #[inline]
    fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    #[inline]
    fn set_range(&mut self, range: Range<usize>) {
        self.range = range
    }

    #[inline]
    fn token_ahead(&self) -> Option<&Token<'i, I, TK>> {
        self.token_ahead.as_ref()
    }

    #[inline]
    fn set_token_ahead(&mut self, token: Token<'i, I, TK>) {
        self.token_ahead = Some(token)
    }

    #[inline]
    fn layout_ahead(&self) -> Option<&'i I> {
        self.layout_ahead
    }

    #[inline]
    fn set_layout_ahead(&mut self, layout: Option<&'i I>) {
        self.layout_ahead = layout
    }
}
