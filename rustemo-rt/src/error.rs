use crate::location::Location;
use std::fmt::Display;

pub type Result<R> = std::result::Result<R, Error>;

#[derive(Debug)]
pub enum Error {
    /// Generic Rustemo error
    Error(String),

    ParseError {
        message: String,
        file: String,
        location: Location,
    },
    IOError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError {
                message,
                file,
                location,
            } => write!(f, "Parse error at {}:{}: {}", file, location, message),
            Error::IOError(e) => write!(f, "IOError: {}", e),
            Error::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
