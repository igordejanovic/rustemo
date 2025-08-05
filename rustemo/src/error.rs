use crate::{position::SourceSpan, Context, Input, State};
use std::fmt::{Debug, Display};

pub type Result<R> = std::result::Result<R, Error>;

/// Error type returned in `Err` variant of `Result` type from the parser.
// ANCHOR: parser-error
#[derive(Debug, thiserror::Error)]
pub struct ParseError {
    pub message: String,

    // FIXME: This should be borrowed when error recovery is implemented.
    pub src: Option<String>,

    pub file: Option<String>,
    pub span: Option<SourceSpan>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut loc_str = String::from("Error");
        if self.file.is_some() || self.span.is_some() {
            loc_str.push_str(" at ");
        }
        if let Some(ref file) = self.file {
            loc_str.push_str(file);
            if self.span.is_some() {
                loc_str.push(':');
            }
        }
        if let Some(span) = self.span {
            loc_str.push_str(&format!("{span:?}"));
        }
        write!(f, "{}:\n\t{}", loc_str, self.message.replace('\n', "\n\t"))
    }
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(Box<ParseError>),

    #[error("{0}")]
    IOError(#[from] std::io::Error),
}
// ANCHOR_END: parser-error

impl Error {
    /// Adds source string to the error.
    pub fn with_source(self, src: String) -> Self {
        match self {
            Error::ParseError(mut parse_error) => {
                parse_error.src = Some(src);
                Error::ParseError(parse_error)
            }
            e => e,
        }
    }

    /// A string representation of the error without the full file path.
    /// Used in tests to yield the same results at different location.
    pub fn to_pos_str(&self) -> String {
        match self {
            Error::ParseError(e) => {
                let ParseError {
                    ref message,
                    ref file,
                    src: _,
                    span,
                } = **e;
                let mut loc_str = String::from("Error");
                if file.is_some() || span.is_some() {
                    loc_str.push_str(" at ");
                }
                if let Some(file) = file {
                    if let Some((_, file)) = file.rsplit_once('/') {
                        loc_str.push_str(file);
                    } else {
                        loc_str.push_str(file);
                    }
                    if span.is_some() {
                        loc_str.push(':');
                    }
                }
                if let Some(span) = span {
                    loc_str.push_str(&format!("{span:?}"));
                }
                format!("{}:\n\t{}", loc_str, message.replace('\n', "\n\t"))
            }
            Error::IOError(e) => format!("IOError: {e}"),
        }
    }
}


impl From<std::io::Error> for Box<Error> {
    fn from(e: std::io::Error) -> Self {
        Box::new(Error::IOError(e))
    }
}

impl<R> From<ParseError> for Result<R> {
    fn from(e: ParseError) -> Self {
        Self::Err(Error::ParseError(Box::new(e)))
    }
}

impl<R> From<Error> for Result<R> {
    fn from(e: Error) -> Self {
        Self::Err(e)
    }
}

pub(crate) fn error_expected<'i, I, S, TK, C>(
    input: &'i I,
    file_name: &str,
    context: &C,
    expected: &[TK],
) -> Error
where
    C: Context<'i, I, S, TK>,
    I: Input + ?Sized,
    S: State,
    TK: Debug,
{
    let expected = if expected.len() > 1 {
        format!(
            "one of {}",
            expected
                .iter()
                .map(|t| format!("{t:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    } else {
        format!("{:?}", expected[0])
    };
    Error::ParseError(Box::new(ParseError {
        message: format!("Expected {}.", expected),
        file: Some(file_name.to_string()),
        src: input.try_to_string(),
        span: Some(context.position().into()),
    }))
}

/// Creates error Result from message, file and span
#[macro_export]
macro_rules! err {
    ($message:expr) => {
        Result::from($crate::ParseError {
            message: $message,
            file: None,
            src: None,
            span: None,
        })
    };
    ($message:expr, $file:expr) => {
        Result::from($crate::ParseError {
            message: $message,
            file: $file,
            src: None,
            span: None,
        })
    };
    ($message:expr, $file:expr, $span:expr) => {
        Result::from($crate::ParseError {
            message: $message,
            file: $file,
            src: None,
            span: $span,
        })
    };
}
