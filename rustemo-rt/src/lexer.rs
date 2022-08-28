use crate::{
    error::Result,
    grammar::TerminalInfo,
    index::TermIndex,
    location::{LineBased, Location, Position},
    log,
};
use core::fmt::Debug;
use std::cmp::min;

/// The `Lexer` trait allows input tokenization
///
/// Lexer is stateless and its job is to produce next token given the current
/// context.
pub trait Lexer<I, LO, ST> {
    /// Given the current context, this method should return RustemoResult with
    /// token found ahead of the current location or error indicating what is
    /// expected.
    fn next_token(&self, context: &mut Context<I, LO, ST>) -> Result<Token<I>>;
}

/// `Token` represent a single token from the input stream.
#[derive(Debug)]
pub struct Token<I> {
    pub terminal: &'static TerminalInfo,

    /// The part of the input stream that this token represents.
    pub value: I,

    /// Location (with span) in the input file where this token is found.
    pub location: Option<Location>,
}

impl<I> Token<I> {
    #[inline]
    pub fn index(&self) -> TermIndex {
        self.terminal.id
    }
}

/// Lexer context is used to keep the lexing state. It provides necessary
/// information to parsers and actions.
#[derive(Debug)]
pub struct Context<I, LO, ST> {
    /// File path of the parsed content. "<str>" In case of static string.
    pub file: String,

    /// The input being parsed. Should be set when the context is created.
    pub input: I,

    /// An absolute position in the input sequence
    ///
    /// The input must be indexable type.
    pub position: usize,


    /// Location in the input if the input is line/column based.
    ///
    /// The location should be tracked by lexer but the lexers are stateless so
    /// the current location is always kept in the context. As location tracking
    /// incurs an overhead it should be configurable. In case of an error, lexer
    /// can calculate location information based on the absolute position.
    pub location: Option<Location>,

    /// Layout before the current token ahead (e.g. whitespaces, comments...)
    pub layout: Option<LO>,

    /// An arbitrary state used by the parser. E.g. for LR it is the current
    /// state of the autmaton.
    pub state: ST,
}

impl<I, LO, ST: Default> Context<I, LO, ST> {
    pub fn new(file: String, input: I) -> Self {
        Self {
            file,
            input,
            position: 0,
            location: None,
            layout: None,
            state: ST::default(),
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

impl<'i, LO, ST: Default> Context<&'i str, LO, ST> {
    pub fn context_str(&self) -> String {
        self.input[self.position - min(15, self.position)..self.position]
            .chars()
            .chain("-->".chars())
            .chain(self.input[self.position..].chars().take(15))
            .collect::<String>()
    }

    pub fn update_location<C: AsRef<str>>(&mut self, content: C) {
        let content = content.as_ref();
        let (mut line, mut column) =
            self.location.map_or((1, 0), |l| match l {
                Location {
                    start: Position::LineBased(lb),
                    ..
                } => (lb.line, lb.column),
                _ => panic!(),
            });
        let newlines =
            content.as_bytes().iter().filter(|&c| *c == b'\n').count();
        let newcolumn = content.len()
            - content
                .as_bytes()
                .iter()
                .rposition(|&c| c == b'\n')
                .unwrap_or(0);
        line += newlines;
        column += newcolumn;

        self.location = Some(Location {
            start: Position::LineBased(LineBased { line, column }),
            end: None,
        });
        self.position = self.position + content.len();
        log!("Position: {}", self.position);
    }
}

impl<I, LO, ST: Default> From<&mut Context<I, LO, ST>> for Location {
    fn from(context: &mut Context<I, LO, ST>) -> Self {
        context.location.unwrap_or(Self {
            start: Position::Position(context.position),
            end: None,
        })
    }
}
