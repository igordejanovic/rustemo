use crate::debug::log;
use crate::error::{Error, Result};
use crate::index::{StateIndex, TermIndex};
use crate::lexer::{AsStr, Context, Input, Lexer, Token, TokenKind};
use crate::location::Location;

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct LRStringLexer<D: 'static> {
    definition: &'static D,
    partial_parse: bool,
    skip_ws: bool,
}

impl<D> LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    pub fn new(
        definition: &'static D,
        partial_parse: bool,
        skip_ws: bool,
    ) -> Self {
        Self {
            definition,
            partial_parse,
            skip_ws,
        }
    }

    fn skip(context: &mut Context<str, StateIndex>) {
        let skipped_len = context.input[context.position..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .count();
        let skipped =
            &context.input[context.position..context.position + skipped_len];
        log!("Skipped ws: {}", skipped.len());
        if skipped_len > 0 {
            context.layout = Some(skipped);
            context.position += skipped_len;
        }
        context.location = Some(skipped.new_location(context.location));
    }
}

impl<'i, D, TK> Lexer<'i, str, StateIndex, TK> for LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
    TK: From<TermIndex> + AsStr + Copy,
{
    fn next_token(
        &self,
        context: &mut Context<'i, str, StateIndex>,
    ) -> Result<Token<'i, str, TK>> {
        if self.skip_ws {
            Self::skip(context);
        }
        log!(
            "Trying recognizers: {:?}",
            self.definition
                .recognizers(context.state)
                .map(|(_, term_idx)| TokenKind::<TK>::from(term_idx).as_str())
                .collect::<Vec<_>>()
        );
        let token: Option<Token<'i, str, TK>> = self
            .definition
            .recognizers(context.state)
            .map(|(recognizer, token_kind)| {
                (recognizer(&context.input[context.position..]), token_kind)
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, token_kind)| Token {
                kind: token_kind.into(),
                value: recognized.unwrap(),
                location: None,
            })
            // Take the first token or return None if no tokens are found.
            .next();

        match token {
            Some(t) => Ok(t),
            None => {
                if self.partial_parse {
                    // If partial parse is configured we shall return STOP when
                    // no new tokens can be found to try to complete what we
                    // have seen so far.
                    Ok(Token {
                        kind: TokenKind::STOP,
                        value: "",
                        location: None,
                    })
                } else {
                    let expected = self
                        .definition
                        .recognizers(context.state)
                        .map(|(_, term_idx)| {
                            TokenKind::<TK>::from(term_idx).as_str()
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    Err(Error::ParseError {
                        message: format!(
                            r#"Error at position {} "{}". Expected one of {}."#,
                            context.location_str(),
                            context.input.context_str(context.position),
                            expected
                        ),
                        file: context.file(),
                        location: Location::from(context),
                    })
                }
            }
        }
    }
}

pub trait LexerDefinition {
    type Recognizer;
    /// For the given state, returns iterator of recognizers that should be
    /// tried in order.
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::Recognizer>;
}

pub struct RecognizerIterator<R: 'static> {
    pub terminals_for_state: &'static [Option<usize>],
    pub recognizers: &'static [R],
    pub index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = (&'static R, TermIndex);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.terminals_for_state.len() {
            match self.terminals_for_state[self.index] {
                Some(term_idx) => {
                    self.index += 1;
                    Some((&self.recognizers[term_idx], TermIndex(term_idx)))
                }
                None => None,
            }
        } else {
            None
        }
    }
}
