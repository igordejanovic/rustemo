use crate::{
    context::Context, input::Input, location::Location, log, parser::State,
};
#[cfg(debug_assertions)]
use colored::*;
use core::fmt::Debug;
use std::marker::PhantomData;

/// The trait implemented by all Rustemo lexers
///
/// Lexer is stateless and its job is to produce the next token given the
/// current context.
///
/// # Generic types
///
/// - `C` - parsing context
/// - `S` - parser state type.
/// - `TK` - token kind type. This is generated by the parser generator from the
///   grammar. This is the type that describes the kinds of token lexer can
///   produce.
///
pub trait Lexer<'i, C, S, TK>
where
    C: Context<'i, Self::Input, S, TK>,
    S: State,
{
    type Input: Input + ?Sized;

    // TODO: Optimization. Find an ergonomic way to return iterator from
    // next_tokens without boxing and using an indirection through trait object
    //type TokenIterator: Iterator<Item = Token<'i, Self::Input, TK>>;

    /// Given the current context, this method yield an iterator over possible
    /// tokens found at the current location where the order and kinds of token
    /// to look for is given by the `token_kinds`.
    ///
    /// Context is mutable to support lexers that implement skipping of
    /// whitespaces.
    fn next_tokens<'a>(
        &self,
        context: &mut C,
        input: &'i Self::Input,
        token_kinds: &'a [Option<TK>],
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TK>> + 'i>
    where
        'a: 'i;
}

/// The trait implemented by types used to recognize tokens in string inputs.
/// Used by [`StringLexer`].
pub trait TokenRecognizer<'i> {
    fn recognize(&self, _input: &'i str) -> Option<&'i str> {
        panic!("Recognize is not defined.")
    }
}

/// A lexer that operates over string inputs and uses generated string and regex
/// recognizers provided by the parser table.
pub struct StringLexer<C, S, TK, TR: 'static, const TERMINAL_COUNT: usize> {
    skip_ws: bool,
    token_recognizers: &'static [TR; TERMINAL_COUNT],
    phantom: PhantomData<(C, S, TK)>,
}

impl<
        'i,
        C: Context<'i, str, S, TK>,
        S: State,
        TK,
        TR: TokenRecognizer<'i>,
        const TERMINAL_COUNT: usize,
    > StringLexer<C, S, TK, TR, TERMINAL_COUNT>
{
    pub fn new(
        skip_ws: bool,
        token_recognizers: &'static [TR; TERMINAL_COUNT],
    ) -> Self {
        Self {
            skip_ws,
            token_recognizers,
            phantom: PhantomData,
        }
    }

    fn skip(input: &'i str, context: &mut C) {
        let skipped_len: usize = input[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        if skipped_len > 0 {
            let skipped =
                &input[context.position()..context.position() + skipped_len];
            log!("\t{} {}", "Skipped ws:".bold().green(), skipped_len);
            context.set_layout_ahead(Some(skipped));
            context.set_position(context.position() + skipped_len);
            context.set_location(skipped.location_after(context.location()));
        } else {
            context.set_layout_ahead(None);
        }
    }
}

struct TokenIterator<'i, TR: 'static, TK> {
    input: &'i str,
    position: usize,
    location: Location,
    token_recognizers: Vec<(&'static TR, TK)>,
    index: usize,
}

impl<'i, TR, TK> TokenIterator<'i, TR, TK> {
    fn new(
        input: &'i str,
        position: usize,
        location: Location,
        token_recognizers: Vec<(&'static TR, TK)>,
    ) -> Self {
        Self {
            input,
            position,
            location,
            token_recognizers,
            index: 0,
        }
    }
}

impl<'i, TK, TR> Iterator for TokenIterator<'i, TR, TK>
where
    TR: TokenRecognizer<'i>,
    TK: Copy,
{
    type Item = Token<'i, str, TK>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index < self.token_recognizers.len() {
                let (recognizer, token_kind) =
                    &self.token_recognizers[self.index];
                self.index += 1;
                if let Some(recognized) =
                    recognizer.recognize(&self.input[self.position..])
                {
                    return Some(Token {
                        kind: *token_kind,
                        value: recognized,
                        location: recognized.location_span(self.location),
                    });
                }
            } else {
                return None;
            }
        }
    }
}

impl<'i, C, S, TK, TR, const TERMINAL_COUNT: usize> Lexer<'i, C, S, TK>
    for StringLexer<C, S, TK, TR, TERMINAL_COUNT>
where
    C: Context<'i, str, S, TK>,
    S: State + Into<usize>,
    TK: Debug + Into<usize> + Copy,
    TR: TokenRecognizer<'i>,
{
    type Input = str;

    fn next_tokens<'a>(
        &self,
        context: &mut C,
        input: &'i Self::Input,
        token_kinds: &'a [Option<TK>],
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TK>> + 'i>
    where
        'a: 'i,
    {
        if self.skip_ws {
            Self::skip(input, context);
        }
        log!(
            "  {} {:?}",
            "Trying recognizers:".green(),
            token_kinds.iter().flatten().collect::<Vec<_>>()
        );

        Box::new(TokenIterator::new(
            input,
            context.position(),
            context.location(),
            token_kinds
                .iter()
                .take_while(|t| t.is_some())
                .map(|tok| {
                    let tok = tok.unwrap();
                    (&self.token_recognizers[tok.into()], tok)
                })
                .collect::<Vec<_>>(),
        ))
    }
}

/// Represents a single token from the input stream.
pub struct Token<'i, I: Input + ?Sized, TK> {
    pub kind: TK,

    /// The part of the input stream that this token represents.
    pub value: &'i I,

    /// Location (with span) in the input file where this token is found.
    pub location: Location,
}

impl<'i, I: Input + ?Sized, TK: Copy> Clone for Token<'i, I, TK> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            value: self.value,
            location: self.location,
        }
    }
}

impl<'i, I, TK> Debug for Token<'i, I, TK>
where
    I: Input + ?Sized,
    I::Output: Debug,
    TK: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({:?} {:?})",
            self.kind,
            if self.value.len() > 50 {
                format!(
                    "{:?}{}{:?}",
                    &self.value.slice(0..20),
                    "..<snip>..",
                    &self.value.slice(self.value.len() - 20..self.value.len())
                )
            } else {
                format!("{:?}", self.value)
            },
            self.location
        )
    }
}
