/// Builds output during parsing by using semantic actions.
///
/// This trait is implemented by types that are in charge of building output of
/// the parsing process (e.g. a parse tree).
pub trait Builder {
    /// A type produced by this builder. See `get_result`.
    type Output;

    /// Returns the product of parsing. Usually the one and only element left on
    /// the result stack.
    fn get_result(&mut self) -> Self::Output;
}
