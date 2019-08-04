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

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        const ALPHABET: [char; 63] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_',
        ];
        const CANNOT_START_WITH: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if string.is_empty() {
            return Err(Error::IsEmpty);
        }

        if let Ok(keyword) = Keyword::from_str(string) {
            return Err(Error::IsKeyword(keyword));
        }

        for (index, character) in string.chars().enumerate() {
            if index == 0 && CANNOT_START_WITH.contains(&character) {
                return Err(Error::CannotStartWith(character));
            }

            if !ALPHABET.contains(&character) {
                return Err(Error::InvalidCharacter(index + 1, character));
            }
        }

        Ok(Self {
            0: string.to_owned(),
        })
    }
}
