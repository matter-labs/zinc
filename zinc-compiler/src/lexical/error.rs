//!
//! The lexical error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnterminatedBlockComment {
        start: Location,
        end: Location,
    },
    UnterminatedDoubleQuoteString {
        start: Location,
        end: Location,
    },
    ExpectedOneOf {
        location: Location,
        expected: String,
        found: char,
    },
    ExpectedOneOfDecimal {
        location: Location,
        expected: String,
        found: char,
    },
    ExpectedOneOfHexadecimal {
        location: Location,
        expected: String,
        found: char,
    },
    InvalidCharacter {
        location: Location,
        found: char,
    },
    UnexpectedEnd {
        location: Location,
    },
}

impl Error {
    pub fn unterminated_block_comment(start: Location, end: Location) -> Self {
        Self::UnterminatedBlockComment { start, end }
    }

    pub fn unterminated_double_quote_string(start: Location, end: Location) -> Self {
        Self::UnterminatedDoubleQuoteString { start, end }
    }

    pub fn expected_one_of(location: Location, expected: Vec<char>, found: char) -> Self {
        Self::ExpectedOneOf {
            location,
            expected: Self::join_expected(expected),
            found,
        }
    }

    pub fn expected_one_of_decimal(location: Location, found: char) -> Self {
        Self::ExpectedOneOfDecimal {
            location,
            expected: Self::join_expected(vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_',
            ]),
            found,
        }
    }

    pub fn expected_one_of_hexadecimal(location: Location, found: char) -> Self {
        Self::ExpectedOneOfHexadecimal {
            location,
            expected: Self::join_expected(vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                'A', 'B', 'C', 'D', 'E', 'F', '_',
            ]),
            found,
        }
    }

    pub fn invalid_character(location: Location, found: char) -> Self {
        Self::InvalidCharacter { location, found }
    }

    pub fn unexpected_end(location: Location) -> Self {
        Self::UnexpectedEnd { location }
    }

    fn join_expected(chars: Vec<char>) -> String {
        chars
            .into_iter()
            .map(|symbol| format!("`{}`", symbol))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
