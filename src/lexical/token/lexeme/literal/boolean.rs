//!
//! The boolean literal lexeme.
//!

use std::convert::TryFrom;
use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Keyword;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Boolean {
    True,
    False,
}

impl TryFrom<Keyword> for Boolean {
    type Error = Keyword;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        Ok(match keyword {
            Keyword::True => Boolean::True,
            Keyword::False => Boolean::False,

            unknown => return Err(unknown),
        })
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Boolean::True => "true",
                Boolean::False => "false",
            }
        )
    }
}
