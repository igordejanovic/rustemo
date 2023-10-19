use std::ops::Range;

use crate::{input::Input, lexer::Token, location::Location, parser::State};

/// Lexer/Parser context is used to keep the state. It provides necessary
/// information to parsers and actions.
pub trait Context<'i, I: Input + ?Sized, S: State, TK> {
    /// The current parser state.
    fn state(&self) -> S;
    fn set_state(&mut self, state: S);

    /// An absolute position in the input sequence
    ///
    /// The input must be indexable type.
    fn position(&self) -> usize;
    fn set_position(&mut self, position: usize);

    /// A span in the input sequence, possibly in line-column style.
    fn location(&self) -> Location;
    fn set_location(&mut self, location: Location);

    /// A span in the input sequence
    fn range(&self) -> Range<usize>;
    fn set_range(&mut self, range: Range<usize>);

    /// Next token recognized in the input at the current parsing location
    fn token_ahead(&self) -> Option<&Token<'i, I, TK>>;
    fn set_token_ahead(&mut self, token: Token<'i, I, TK>);

    /// A layout before the token ahead
    fn layout_ahead(&self) -> Option<&'i I>;
    fn set_layout_ahead(&mut self, layout: Option<&'i I>);
}
