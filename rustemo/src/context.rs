use crate::{input::Input, lexer::Token, parser::State, position::SourceSpan, Position};

/// Lexer/Parser context is used to keep the state. It provides necessary
/// information to parsers and actions.
pub trait Context<'i, I: Input + ?Sized, S: State, TK>: Default {
    /// The current parser state.
    fn state(&self) -> S;
    fn set_state(&mut self, state: S);

    /// A position in the input sequence.
    ///
    /// The input must be indexable type.
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);

    /// A span in the input sequence.
    fn span(&self) -> SourceSpan;
    fn set_span(&mut self, span: SourceSpan);

    /// Next token recognized in the input at the current parsing location
    fn token_ahead(&self) -> Option<&Token<'i, I, TK>>;
    fn set_token_ahead(&mut self, token: Token<'i, I, TK>);

    /// A layout before the token ahead
    fn layout_ahead(&self) -> Option<&'i I>;
    fn set_layout_ahead(&mut self, layout: Option<&'i I>);
}
