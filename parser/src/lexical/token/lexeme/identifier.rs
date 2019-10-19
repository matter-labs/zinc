//!
//! The lexical token identifier lexeme.
//!

use std::convert::TryFrom;
use std::fmt;

use failure::Fail;

use crate::lexical::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "is empty")]
    IsEmpty,
    #[fail(display = "is underscore")]
    IsUnderscore,
    #[fail(display = "is keyword: {:?}", _0)]
    IsKeyword(Keyword),
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn can_start_with(character: char) -> bool {
        character.is_ascii_alphabetic() || character == '_'
    }

    pub fn can_contain_since_index_1(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '_'
    }
}

impl TryFrom<&str> for Identifier {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.is_empty() {
            return Err(Error::IsEmpty);
        }

        if input == "_" {
            return Err(Error::IsUnderscore);
        }

        if let Ok(keyword) = Keyword::try_from(input) {
            return Err(Error::IsKeyword(keyword));
        }

        Ok(Self {
            name: input.to_owned(),
        })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
