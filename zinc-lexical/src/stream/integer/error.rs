//!
//! The lexical integer literal parser error.
//!

///
/// The lexical integer literal parser error.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// The lexeme is not an integer, which means that another parser must be run.
    NotAnInteger,
    /// The lexeme is `0b`, which is not a valid binary literal.
    EmptyBinaryBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// The lexeme is `0o`, which is not a valid octal literal.
    EmptyOctalBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// The decimal literal exponent cannot be empty.
    EmptyExponent {
        /// The position of the invalid character.
        offset: usize,
    },
    /// The lexeme is `0x`, which is not a valid hexadecimal literal.
    EmptyHexadecimalBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// A non-binary character is found in a binary literal.
    ExpectedOneOfBinary {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-octal character is found in an octal literal.
    ExpectedOneOfOctal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-decimal character is found in a decimal literal.
    ExpectedOneOfDecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-hexadecimal character is found in a hexadecimal literal.
    ExpectedOneOfHexadecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// Unable to finish a literal.
    UnexpectedEnd,
}
