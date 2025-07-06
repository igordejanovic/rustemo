use crate::{
    context::Context, input::Input, lexer::Token, parser::State, position::SourceSpan, Position,
};

/// [`Context`] implementation for LR parsing
#[derive(Debug)]
pub struct LRContext<'i, I: Input + ?Sized, S, TK> {
    position: Position,

    /// The span of token/non-terminal during shift/reduce operation.
    span: SourceSpan,

    /// Layout before the lookahead token (e.g. whitespaces, comments...)
    layout_ahead: Option<&'i I>,

    token_ahead: Option<Token<'i, I, TK>>,

    state: S,
}

impl<I: Input + ?Sized, S: Default, TK> Default for LRContext<'_, I, S, TK> {
    fn default() -> Self {
        Self::new(I::start_position())
    }
}

impl<I: Input + ?Sized, S: Default, TK> LRContext<'_, I, S, TK> {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            span: SourceSpan {
                start: position,
                end: position,
            },
            layout_ahead: None,
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
    fn position(&self) -> Position {
        self.position
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    #[inline]
    fn span(&self) -> SourceSpan {
        self.span
    }

    #[inline]
    fn set_span(&mut self, location: SourceSpan) {
        self.span = location
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
