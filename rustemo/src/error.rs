use crate::{location::Location, Context, Input, State};
use std::fmt::{Debug, Display};

pub type Result<R> = std::result::Result<R, Error>;

/// Error type returned in `Err` variant of `Result` type from the parser.
// ANCHOR: parser-error
#[derive(Debug)]
pub enum Error {
    Error {
        message: String,
        file: Option<String>,
        location: Option<Location>,
    },
    IOError(std::io::Error),
}
// ANCHOR_END: parser-error

impl Error {
    /// A string representation of the error without the full file path.
    /// Used in tests to yield the same results at different location.
    pub fn to_locfile_str(&self) -> String {
        match self {
            Error::Error {
                message,
                file,
                location,
            } => {
                let mut loc_str = String::from("Error");
                if file.is_some() || location.is_some() {
                    loc_str.push_str(" at ");
                }
                if let Some(file) = file {
                    if let Some((_, file)) = file.rsplit_once('/') {
                        loc_str.push_str(file);
                    } else {
                        loc_str.push_str(file);
                    }
                    if location.is_some() {
                        loc_str.push(':');
                    }
                }
                if let Some(location) = location {
                    loc_str.push_str(&format!("{location:?}"));
                }
                format!("{}:\n\t{}", loc_str, message.replace('\n', "\n\t"))
            }
            Error::IOError(e) => format!("IOError: {}", e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error {
                message,
                file,
                location,
            } => {
                let mut loc_str = String::from("Error");
                if file.is_some() || location.is_some() {
                    loc_str.push_str(" at ");
                }
                if let Some(file) = file {
                    loc_str.push_str(file);
                    if location.is_some() {
                        loc_str.push(':');
                    }
                }
                if let Some(location) = location {
                    loc_str.push_str(&format!("{location:?}"));
                }
                write!(f, "{}:\n\t{}", loc_str, message.replace('\n', "\n\t"))
            }
            Error::IOError(e) => write!(f, "IOError: {}", e),
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
        location: Some(context.location()),
    }
}

/// Creates error Result from message, file and location
#[macro_export]
macro_rules! err {
    ($message:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: None,
            location: None,
        })
    };
    ($message:expr, $file:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: $file,
            location: None,
        })
    };
    ($message:expr, $file:expr, $location:expr) => {
        Result::from(Error::Error {
            message: $message,
            file: $file,
            location: $location,
        })
    };
}
