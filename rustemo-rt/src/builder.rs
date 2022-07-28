use crate::error::RustemoResult;

/// Building output during semantic actions.
///
/// This trait is implemented by types that are in charge of building output of
/// the parsing process (e.g. a parse tree).
pub trait Builder {

    /// A type produced by this builder. See `get_result`.
    type Output;

    fn new() -> Self;

    /// Returns the product of parsing. Usually the one and only element left on
    /// the result stack.
    fn get_result(&mut self) -> RustemoResult<Self::Output>;
}

