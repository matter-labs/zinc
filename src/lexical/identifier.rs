//!
//! The syntax identifier.
//!

use std::str::FromStr;

use failure::Fail;
use serde_derive::Serialize;

use crate::syntax::Keyword;

#[derive(Debug, Serialize)]
pub struct Identifier(String);

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "is empty")]
    IsEmpty,
    #[fail(display = "is keyword: {:?}", _0)]
    IsKeyword(Keyword),
    #[fail(display = "cannot start with: {}", _0)]
    CannotStartWith(char),
    #[fail(display = "invalid character at position {}: {}", _0, _1)]
    InvalidCharacter(usize, char),
}

impl Identifier {
    pub fn is_in_alphabet(c: char) -> bool {
        ('0' <= c && c <= '9') || ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z') || c == '_'
    }

    pub fn can_start_with(c: char) -> bool {
        ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z') || c == '_'
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(Error::IsEmpty);
        }

        if let Ok(keyword) = Keyword::from_str(string) {
            return Err(Error::IsKeyword(keyword));
        }

        for (index, character) in string.chars().enumerate() {
            if index == 0 && !Self::can_start_with(character) {
                return Err(Error::CannotStartWith(character));
            }

            if Self::is_in_alphabet(character) {
                return Err(Error::InvalidCharacter(index + 1, character));
            }
        }

        Ok(Self {
            0: string.to_owned(),
        })
    }
}
