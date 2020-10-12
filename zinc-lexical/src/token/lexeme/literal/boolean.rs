//!
//! The lexical token boolean literal lexeme.
//!

use std::convert::TryFrom;
use std::fmt;

use crate::token::lexeme::keyword::Keyword;

///
/// The lexical boolean literal.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    /// Created from the `false` keyword.
    False,
    /// Created from the `true` keyword.
    True,
}

impl Boolean {
    ///
    /// Creates a `false` value.
    ///
    pub fn r#false() -> Self {
        Self::False
    }

    ///
    /// Creates a `true` value.
    ///
    pub fn r#true() -> Self {
        Self::True
    }
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

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        match self {
            Self::False => false,
            Self::True => true,
        }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
        }
    }
}
