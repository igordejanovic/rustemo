use crate::{
    error::Result,
    index::TermIndex,
    location::{LineBased, Location, Position},
};
use core::fmt::Debug;
use std::{cmp::min, fmt::Display, iter::once, ops::Range};

/// The `Lexer` trait allows input tokenization
///
/// Lexer is stateless and its job is to produce next token given the current
/// context.
pub trait Lexer<'i, I: Input + ?Sized, ST, TK> {
    /// Given the current context, this method should return a result with token
    /// found ahead of the current location or error indicating what is
    /// expected.
    fn next_token(
        &self,
        context: &mut Context<'i, I, ST>,
    ) -> Result<Token<'i, I, TK>>;
}

/// This trait must be implemented by all types that should be parsed by
/// Rustemo. Input is a sequence-like type with a concept of length.
pub trait Input {
    /// Returns a string context for the given position. Used in debugging outputs.
    fn context_str(&self, position: usize) -> String;

    /// Returns the length of the input.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Given the current location returns the location at the end of self.
    /// Location is an input-specific concept. E.g. for text it is line/column.
    fn new_location(&self, location: Option<Location>) -> Location;
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<K> {
    STOP,
    Kind(K),
}

pub trait AsStr {
    fn as_str(&self) -> &'static str;
}

impl<K: AsStr> AsStr for TokenKind<K> {
    fn as_str(&self) -> &'static str {
        match self {
            TokenKind::STOP => "STOP",
            TokenKind::Kind(k) => k.as_str(),
        }
    }
}

impl<K: From<TermIndex>> From<TermIndex> for TokenKind<K> {
    fn from(idx: TermIndex) -> Self {
        if idx.0 == 0 {
            TokenKind::STOP
        } else {
            TokenKind::Kind(K::from(idx))
        }
    }
}

impl<K: Into<TermIndex>> From<TokenKind<K>> for TermIndex {
    fn from(token_kind: TokenKind<K>) -> Self {
        match token_kind {
            TokenKind::STOP => TermIndex(0),
            TokenKind::Kind(k) => k.into(),
        }
    }
}

/// `Token` represent a single token from the input stream.
#[derive(Debug)]
pub struct Token<'i, I: Input + ?Sized, TK> {
    pub kind: TokenKind<TK>,

    /// The part of the input stream that this token represents.
    pub value: &'i I,

    /// Location (with span) in the input file where this token is found.
    pub location: Option<Location>,
}

/// Lexer context is used to keep the lexing state. It provides necessary
/// information to parsers and actions.
#[derive(Debug)]
pub struct Context<'i, I: Input + ?Sized, ST> {
    /// File path of the parsed content. `<str>` In case of static string.
    pub file: String,

    /// The input being parsed. Should be set when the context is created.
    pub input: &'i I,

    /// An absolute position in the input sequence
    ///
    /// The input must be indexable type.
    pub position: usize,

    /// The range of token/non-terminal during shift/reduce operation.
    pub range: Range<usize>,

    /// Location in the input if the input is line/column based.
    ///
    /// The location should be tracked by lexer but the lexers are stateless so
    /// the current location is always kept in the context. As location tracking
    /// incurs an overhead it should be configurable. In case of an error, lexer
    /// can calculate location information based on the absolute position.
    pub location: Option<Location>,

    /// Layout before the current token ahead (e.g. whitespaces, comments...)
    pub layout: Option<&'i I>,

    /// An arbitrary state used by the parser. E.g. for LR it is the current
    /// state of the automaton.
    pub state: ST,
}

impl<'i, I: Input + ?Sized, ST: Default> Context<'i, I, ST> {
    pub fn new(file: String, input: &'i I) -> Self {
        Self {
            file,
            input,
            position: 0,
            location: None,
            layout: None,
            state: ST::default(),
            range: 0..0,
        }
    }

    #[inline]
    pub fn file(&self) -> String {
        self.file.clone()
    }

    #[inline]
    pub fn location_str(&self) -> String {
        match self.location {
            Some(location) => {
                format!("{}:{}", self.file(), location)
            }
            None => format!("{}:{}", self.file(), self.position),
        }
    }
}

impl Input for str {
    fn context_str(&self, position: usize) -> String {
        self[position - min(15, position)..position]
            .chars()
            .chain("-->".chars())
            .chain(self[position..].chars().take(15))
            .collect::<String>()
    }

    fn len(&self) -> usize {
        str::len(self)
    }

    fn new_location(&self, location: Option<Location>) -> Location {
        let (mut line, mut column) = location.map_or((1, 0), |l| match l {
            Location {
                start: Position::LineBased(lb),
                ..
            } => (lb.line, lb.column),
            _ => panic!(),
        });
        let newlines = self.as_bytes().iter().filter(|&c| *c == b'\n').count();
        let newcolumn = self.len()
            - self
                .as_bytes()
                .iter()
                .rposition(|&c| c == b'\n')
                .unwrap_or(0);
        line += newlines;
        column += newcolumn;

        Location {
            start: Position::LineBased(LineBased { line, column }),
            end: None,
        }
    }
}

impl<T> Input for [T]
where
    T: Display,
{
    fn context_str(&self, position: usize) -> String {
        format!(
            "{:?}",
            self[position - min(15, position)..position]
                .iter()
                .map(|x| format!("{x}"))
                .chain(once("-->".to_string()))
                .chain(self[position..].iter().map(|x| format!("{x}")).take(15))
                .collect::<Vec<_>>()
        )
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn new_location(&self, location: Option<Location>) -> Location {
        if let Some(Location {
            start: Position::Position(p),
            ..
        }) = location
        {
            Location {
                start: Position::Position(p + self.len()),
                end: None,
            }
        } else {
            Location {
                start: Position::Position(self.len()),
                end: None,
            }
        }
    }
}

impl<'i, I: Input + ?Sized, ST: Default> From<&mut Context<'i, I, ST>>
    for Location
{
    fn from(context: &mut Context<I, ST>) -> Self {
        context.location.unwrap_or(Self {
            start: Position::Position(context.position),
            end: None,
        })
    }
}
