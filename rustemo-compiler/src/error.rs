pub type Result<R> = std::result::Result<R, Error>;

#[derive(Debug, thiserror::Error)]
#[error("Rustemo error")]
pub enum Error {
    #[error("{0}")]
    RustemoError(Box<rustemo::Error>),

    #[error("Error: {0}")]
    Error(String),

    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Syn error: {0}")]
    SynError(#[from] syn::Error),
}

impl Error {
    /// A string representation of the error without the full file path.
    /// Used in tests to yield the same results at different location.
    pub fn to_locfile_str(&self) -> String {
        match self {
            Error::RustemoError(e) => e.to_pos_str(),
            Error::SynError(e) => format!("Syn error: {e}"),
            Error::IOError(e) => format!("IOError: {e}"),
            Error::Error(e) => format!("Error: {e}"),
        }
    }
}

impl From<rustemo::Error> for Error {
    fn from(e: rustemo::Error) -> Self {
        Error::RustemoError(Box::new(e))
    }
}
