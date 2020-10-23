//!
//! The lexical symbol parser error.
//!

///
/// The lexical symbol parser error.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An unexpected character resulting into an invalid symbol.
    InvalidCharacter {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// Unable to finish a symbol.
    UnexpectedEnd,
}
