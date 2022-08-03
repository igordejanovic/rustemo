pub type Result<R> = std::result::Result<R, Error>;

#[derive(Debug)]
pub enum Error {
    RustemoError(rustemo_rt::Error),
    IOError(std::io::Error),
    SynError(syn::Error),
    Error(String),
}

impl From<rustemo_rt::Error> for Error {
    fn from(e: rustemo_rt::Error) -> Self {
        Error::RustemoError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::SynError(e)
    }
}
