use crate::{
    debug::log,
    grammar::TerminalInfo,
    lexer::{Lexer, Token},
    parser::{Context}, index::StateIndex,
};

pub trait LexerDefinition {
    type Recognizer;
    /// For the given state, returns iterator of recognizers that should be
    /// tried in order.
    fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer>;
}

pub struct RecognizerIterator<R: 'static> {
    pub(crate) terminals: &'static [TerminalInfo],
    pub(crate) terminals_for_state: &'static [Option<usize>],
    pub(crate) recognizers: &'static [R],
    pub(crate) index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = (&'static R, &'static TerminalInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.terminals_for_state.len() {
            match self.terminals_for_state[self.index] {
                Some(term_idx) => {
                    self.index += 1;
                    Some((&self.recognizers[term_idx], &self.terminals[term_idx]))
                }
                None => None,
            }
        } else {
            None
        }
    }
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct DefaultLexer<'i, D: 'static> {
    pub(crate) input: &'i str,
    pub(crate) token_ahead: Option<Token<&'i str>>,
    pub(crate) definition: &'static D,
}

impl<'i, D> DefaultLexer<'i, D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    fn new(input: &'i str, definition: &'static D) -> Self {
        Self {
            input,
            token_ahead: None,
            definition,
        }
    }
    fn skip(&mut self, context: &mut impl Context) {
        let skipped = self.input[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .collect::<String>();
        log!("Skipped ws: {}", skipped.len());
        context.set_position(context.position() + skipped.len());
    }
}

impl<'i, D> Lexer for DefaultLexer<'i, D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    type Input = &'i str;
    fn next_token(&mut self, context: &mut impl Context) -> Option<Token<Self::Input>> {
        self.skip(context);
        log!(
            "Context: {}",
            self.input.chars().take(30).collect::<String>()
        );
        let token: Token<&'i str> = self
            .definition
            .recognizers(context.state())
            .map(|(recognizer, terminal_info)| {
                (recognizer(&self.input[context.position()..]), terminal_info)
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, terminal_info)| Token {
                terminal: terminal_info,
                value: recognized.unwrap(),
                location: None,
                layout: None,
                layout_location: None,
            })
            // Take the first token or return None if no tokens are found.
            .next()?;

        let new_pos = context.position() + token.value.len();
        context.set_position(new_pos);
        self.token_ahead = Some(token);
        self.token_ahead.clone()
    }
}
