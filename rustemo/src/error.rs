use crate::location::Location;
use std::fmt::Display;

pub type Result<R> = std::result::Result<R, Error>;

#[derive(Debug)]
pub enum Error {
    Error {
        message: String,
        file: Option<String>,
        location: Option<Location>,
    },
    IOError(std::io::Error),
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
                    loc_str.push_str(&location.to_string())
                }
                write!(f, "{}:{}", loc_str, message)
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
