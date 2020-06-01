//!
//! The lexical token identifier lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::lexical::token::lexeme::keyword::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub inner: String,
}

#[derive(Debug)]
pub enum Error {
    IsUnderscore,
    IsKeyword(Keyword),
}

pub static STRING_DELIMITER: &str = "_";

impl Identifier {
    pub const CHARACTER_DELIMITER: char = '_';

    pub fn new(inner: String) -> Self {
        Self { inner }
    }

    pub fn can_start_with(character: char) -> bool {
        character.is_ascii_alphabetic() || character == Self::CHARACTER_DELIMITER
    }

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
