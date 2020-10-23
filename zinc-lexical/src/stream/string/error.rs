//!
//! The lexical string literal parser error.
//!

///
/// The lexical string literal parser error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The lexeme is not a string, which means that another parser must be run.
    NotAString,
    /// The string has not been terminated, which ends up with an entire file treated as an unterminated string.
    UnterminatedDoubleQuote {
        /// The number of lines in the unterminated string.
        lines: usize,
        /// The column where the unterminated string ends.
        column: usize,
    },
}
