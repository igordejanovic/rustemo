use std::fmt::Display;
use crate::location::Location;

pub type RustemoResult<R> = Result<R, RustemoError>;

#[derive(Debug)]
pub enum RustemoError {
    /// Generic Rustemo error
    Error(String),

    ParseError {
        message: String,
        file: String,
        location: Location,
    },
    IOError(std::io::Error),
}

impl Display for RustemoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustemoError::ParseError {
                message,
                file,
                location,
            } => write!(f, "Parse error at {}:{}: {}", file, location, message),
            RustemoError::IOError(e) => write!(f, "Error: {}", e),
            RustemoError::Error(e) => write!(f, "IOError: {}", e),
        }
    }
}

impl From<std::io::Error> for RustemoError {
    fn from(e: std::io::Error) -> Self {
        RustemoError::IOError(e)
    }
}
