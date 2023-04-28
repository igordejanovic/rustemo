use crate::debug::log;
use crate::err;
use crate::error::{Error, Result};
use crate::index::{StateIndex, TermIndex};
use crate::lexer::{AsStr, Context, Input, Lexer, Token};
use crate::location::Location;

pub trait StringRecognizer<TK> {
    fn recognize<'i>(&self, input: &'i str) -> Option<&'i str>;
    fn token_kind(&self) -> TK;
    fn finish(&self) -> bool;
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct LRStringLexer<D: 'static> {
    definition: &'static D,
    partial_parse: bool,
    skip_ws: bool,
}

impl<D, TR> LRStringLexer<D>
where
    D: LexerDefinition<TokenRecognizer = TR>,
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
            context.layout_ahead = Some(skipped);
            context.position += skipped_len;
        } else {
            context.layout_ahead = None;
        }
        context.location = skipped.location_after(context.location);
    }
}

impl<D, TK, TR> Lexer<str, StateIndex, TK> for LRStringLexer<D>
where
    D: LexerDefinition<TokenRecognizer = TR>,
    TK: From<TermIndex> + AsStr + Default,
    TR: StringRecognizer<TK> + 'static,
{
    fn next_token<'i>(
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
                .map(|recognizer| recognizer.token_kind().as_str())
                .collect::<Vec<_>>()
        );
        let token: Option<Token<'i, str, TK>> = self
            .definition
            .recognizers(context.state)
            .map(|recognizer| {
                (
                    recognizer.recognize(&context.input[context.position..]),
                    recognizer.token_kind(),
                )
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, token_kind)| Token {
                kind: token_kind,
                value: recognized.unwrap(),
                location: recognized.unwrap().location_span(context.location),
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
                        kind: TK::default(),
                        value: "",
                        location: context.location,
                    })
                } else {
                    let expected = self
                        .definition
                        .recognizers(context.state)
                        .map(|recognizer| recognizer.token_kind().as_str())
                        .collect::<Vec<_>>();
                    let expected = if expected.len() > 1 {
                        format!("one of {}", expected.join(", "))
                    } else {
                        expected[0].into()
                    };
                    err!(
                        format!(
                            "...{}...\nExpected {}.",
                            context.input.context_str(context.position),
                            expected
                        ),
                        Some(context.file.clone()),
                        Some(Location::from(context))
                    )
                }
            }
        }
    }
}

pub trait LexerDefinition {
    type TokenRecognizer;
    /// For the given state, returns iterator of recognizers that should be
    /// tried in order.
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::TokenRecognizer>;
}

pub struct RecognizerIterator<R: 'static> {
    pub token_rec_for_state: &'static [Option<R>],
    pub index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = &'static R;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.token_rec_for_state.len() {
            self.index += 1;
            self.token_rec_for_state[self.index - 1].as_ref()
        } else {
            None
        }
    }
}
