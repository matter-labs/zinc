//!
//! The lexical token identifier lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::token::lexeme::keyword::Keyword;

///
/// An identifier lexeme, which is usually used to name an item.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// The identifier string contents.
    pub inner: String,
}

///
/// The identifier parsing error.
///
/// If the parser returns such an error, it means that the word is not a valid identifier,
/// but a keyword, underscore symbol, or something else.
///
#[derive(Debug)]
pub enum Error {
    /// The word consists of a single `_`.
    IsUnderscore,
    /// The word is a keyword.
    IsKeyword(Keyword),
}

/// The underscore string, which is not a valid identifier.
pub static STRING_DELIMITER: &str = "_";

impl Identifier {
    /// The underscore character, which can appear at the beginning of an identifier.
    pub const CHARACTER_DELIMITER: char = '_';

    ///
    /// Creates an identifier value.
    ///
    pub fn new(inner: String) -> Self {
        Self { inner }
    }

    ///
    /// Checks if identifier can have the `character` at the first position.
    ///
    /// Only alphabetic characters and the underscore character are allowed.
    ///
    pub fn can_start_with(character: char) -> bool {
        character.is_ascii_alphabetic() || character == Self::CHARACTER_DELIMITER
    }

    ///
    /// Checks if identifier can have the `character` at the second position.
    ///
    /// Only alphanumeric characters and the underscore character are allowed.
    ///
    pub fn can_contain_after_start(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == Self::CHARACTER_DELIMITER
    }
}

impl TryFrom<String> for Identifier {
    type Error = Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::from_str(input.as_str())
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == STRING_DELIMITER {
            return Err(Error::IsUnderscore);
        }

        if let Ok(keyword) = Keyword::try_from(input) {
            return Err(Error::IsKeyword(keyword));
        }

        Ok(Self {
            inner: input.to_owned(),
        })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
