use crate::{position::SourceSpan, Context, Input, Position, State};
use std::fmt::{Debug, Display};

pub type Result<R> = std::result::Result<R, Error>;

/// Error type returned in `Err` variant of `Result` type from the parser.
// ANCHOR: parser-error
#[derive(Debug)]
pub enum Error {
    Error {
        message: String,
        file: Option<String>,
        position: Option<Position>,
        span: Option<SourceSpan>,
    },
    IOError(std::io::Error),
}
// ANCHOR_END: parser-error

impl Error {
    /// A string representation of the error without the full file path.
    /// Used in tests to yield the same results at different location.
    pub fn to_pos_str(&self) -> String {
        match self {
            Error::Error {
                message,
                file,
                position,
                span,
            } => {
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
                } else if let Some(position) = position {
                    loc_str.push_str(&format!("{position:?}"));
                }
                format!("{}:\n\t{}", loc_str, message.replace('\n', "\n\t"))
            }
            Error::IOError(e) => format!("IOError: {e}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error {
                message,
                file,
                position,
                span,
            } => {
                let mut loc_str = String::from("Error");
                if file.is_some() || span.is_some() {
                    loc_str.push_str(" at ");
                }
                if let Some(file) = file {
                    loc_str.push_str(file);
                    if span.is_some() || position.is_some() {
                        loc_str.push(':');
                    }
                }
                if let Some(span) = span {
                    loc_str.push_str(&format!("{span:?}"));
                } else if let Some(position) = position {
                    loc_str.push_str(&format!("{position:?}"));
                }
                write!(f, "{}:\n\t{}", loc_str, message.replace('\n', "\n\t"))
            }
            Error::IOError(e) => write!(f, "IOError: {e}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl<R> From<Error> for Result<R> {
    fn from(value: Error) -> Self {
        Self::Err(value)
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
    Error::Error {
        message: format!(
            "...{}...\nExpected {}.",
            input.context_str(context.position()),
            expected
        ),
        file: Some(file_name.to_string()),
        position: Some(context.position()),
        span: None,
    }
}

/// Creates error Result from message, file and span
#[macro_export]
macro_rules! err {
    ($message:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: None,
            position: None,
            span: None,
        })
    };
    ($message:expr, $file:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: $file,
            position: None,
            span: None,
        })
    };
    ($message:expr, $file:expr, $position:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: $file,
            position: $position,
            span: None,
        })
    };
    ($message:expr, $file:expr, ,$span:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: $file,
            position: None,
            span: $span,
        })
    };
}
