//!
//! The lexical parser error.
//!

use crate::token::lexeme::literal::integer::Integer;
use crate::token::location::Location;

///
/// The lexical parser error.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// The comment has not been terminated, which ends up with an entire file treated as an unterminated comment.
    UnterminatedBlockComment {
        /// The location where the unterminated comment starts.
        start: Location,
        /// The location where the unterminated comment ends.
        end: Location,
    },
    /// The string has not been terminated, which ends up with an entire file treated as an unterminated string.
    UnterminatedDoubleQuoteString {
        /// The location where the unterminated string starts.
        start: Location,
        /// The location where the unterminated string ends.
        end: Location,
    },
    /// A non-binary character is found in a binary literal.
    ExpectedOneOfBinary {
        /// The location of the invalid character.
        location: Location,
        /// The allowed characters.
        expected: String,
        /// The invalid character.
        found: char,
    },
    /// A non-octal character is found in an octal literal.
    ExpectedOneOfOctal {
        /// The location of the invalid character.
        location: Location,
        /// The allowed characters.
        expected: String,
        /// The invalid character.
        found: char,
    },
    /// A non-decimal character is found in a decimal literal.
    ExpectedOneOfDecimal {
        /// The location of the invalid character.
        location: Location,
        /// The allowed characters.
        expected: String,
        /// The invalid character.
        found: char,
    },
    /// A non-hexadecimal character is found in a hexadecimal literal.
    ExpectedOneOfHexadecimal {
        /// The location of the invalid character.
        location: Location,
        /// The allowed characters.
        expected: String,
        /// The invalid character.
        found: char,
    },
    /// An unexpected character forbidden in the current state.
    InvalidCharacter {
        /// The location of the invalid character.
        location: Location,
        /// The invalid character.
        found: char,
    },
    /// Unable to finish a lexeme.
    UnexpectedEnd {
        /// The location of the end of the file.
        location: Location,
    },
}

impl Error {
    ///
    /// A shortcut constructor.
    ///
    pub fn unterminated_block_comment(start: Location, end: Location) -> Self {
        Self::UnterminatedBlockComment { start, end }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn unterminated_double_quote_string(start: Location, end: Location) -> Self {
        Self::UnterminatedDoubleQuoteString { start, end }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of_binary(location: Location, found: char) -> Self {
        Self::ExpectedOneOfBinary {
            location,
            expected: Self::join_expected(Integer::CHARACTERS_BINARY.to_vec()),
            found,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of_octal(location: Location, found: char) -> Self {
        Self::ExpectedOneOfOctal {
            location,
            expected: Self::join_expected(Integer::CHARACTERS_OCTAL.to_vec()),
            found,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of_decimal(location: Location, found: char) -> Self {
        Self::ExpectedOneOfDecimal {
            location,
            expected: Self::join_expected(Integer::CHARACTERS_DECIMAL.to_vec()),
            found,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of_hexadecimal(location: Location, found: char) -> Self {
        Self::ExpectedOneOfHexadecimal {
            location,
            expected: Self::join_expected(Integer::CHARACTERS_HEXADECIMAL.to_vec()),
            found,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn invalid_character(location: Location, found: char) -> Self {
        Self::InvalidCharacter { location, found }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn unexpected_end(location: Location) -> Self {
        Self::UnexpectedEnd { location }
    }

    ///
    /// Converts a group of characters into a comma-separated list.
    ///
    /// E.g. ['a', 'b', 'c'] turns into `a`, `b`, `c`.
    ///
    fn join_expected(chars: Vec<char>) -> String {
        chars
            .into_iter()
            .map(|symbol| format!("`{}`", symbol))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
