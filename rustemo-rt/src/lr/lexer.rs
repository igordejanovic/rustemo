use crate::debug::log;
use crate::error::{Error, Result};
use crate::grammar::TerminalInfo;
use crate::index::{StateIndex, TermIndex};
use crate::lexer::{Context, Input, Lexer, Token};
use crate::location::Location;

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct LRStringLexer<D: 'static> {
    definition: &'static D,
    partial_parse: bool,
}

impl<D> LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    pub fn new(definition: &'static D, partial_parse: bool) -> Self {
        Self {
            definition,
            partial_parse,
        }
    }

    fn skip<'i, LO>(context: &mut Context<&'i str, LO, StateIndex>) {
        let skipped = context.input[context.position..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .collect::<String>();
        log!("Skipped ws: {}", skipped.len());
        // context.layout = Some(
        //     &context.input[context.position..context.position + skipped.len()],
        // );
        context.location =
            Some(skipped.as_str().new_location(context.location));
    }
}

impl<'i, D, LO> Lexer<&'i str, LO, StateIndex> for LRStringLexer<D>
where
    D: LexerDefinition<Recognizer = for<'a> fn(&'a str) -> Option<&'a str>>,
{
    fn next_token(
        &self,
        context: &mut Context<&'i str, LO, StateIndex>,
    ) -> Result<Token<&'i str>> {
        Self::skip(context);
        log!(
            "Trying recognizers: {:?}",
            self.definition
                .recognizers(context.state)
                .map(|(_, terminal_info)| terminal_info.name)
                .collect::<Vec<_>>()
        );
        let token: Option<Token<&'i str>> = self
            .definition
            .recognizers(context.state)
            .map(|(recognizer, terminal_info)| {
                (
                    recognizer(&context.input[context.position..]),
                    terminal_info,
                )
            })
            // Skip unsuccesful recognition
            .skip_while(|(recognized, _)| recognized.is_none())
            // Create tokens
            .map(|(recognized, terminal_info)| Token {
                terminal: terminal_info,
                value: recognized.unwrap(),
                location: None,
            })
            // Take the first token or return None if no tokens are found.
            .next();

        match token {
            Some(t) => {
                context.location = Some(t.value.new_location(context.location));
                Ok(t)
            }
            None => {
                if self.partial_parse {
                    // If partial parse is configured we shall return STOP when
                    // no new tokens can be found to try to complete what we
                    // have seen so far.
                    Ok(Token {
                        terminal: &TerminalInfo {
                            id: TermIndex(0),
                            name: "STOP",
                        },
                        value: "",
                        location: None,
                    })
                } else {
                    let expected = self
                        .definition
                        .recognizers(context.state)
                        .map(|(_, terminal_info)| terminal_info.name)
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
    pub terminals: &'static [TerminalInfo],
    pub terminals_for_state: &'static [Option<usize>],
    pub recognizers: &'static [R],
    pub index: usize,
}

impl<R> Iterator for RecognizerIterator<R> {
    type Item = (&'static R, &'static TerminalInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.terminals_for_state.len() {
            match self.terminals_for_state[self.index] {
                Some(term_idx) => {
                    self.index += 1;
                    Some((
                        &self.recognizers[term_idx],
                        &self.terminals[term_idx],
                    ))
                }
                None => None,
            }
        } else {
            None
        }
    }
}
