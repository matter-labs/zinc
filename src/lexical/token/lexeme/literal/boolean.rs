//!
//! The boolean literal lexeme.
//!

use std::convert::TryFrom;
use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Keyword;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "value")]
pub enum Boolean {
    False,
    True,
}

impl TryFrom<Keyword> for Boolean {
    type Error = Keyword;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        Ok(match keyword {
            Keyword::False => Self::False,
            Keyword::True => Self::True,
            unknown => return Err(unknown),
        })
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
        }
    }
}
