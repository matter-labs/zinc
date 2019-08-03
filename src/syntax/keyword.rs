//!
//! The syntax keyword.
//!

use std::str::FromStr;

use crate::syntax::TypeKeyword;

#[derive(Debug)]
pub enum Keyword {
    // domain
    Inputs,
    Witness,
    Require,

    // declaration
    Let,
    Mut,

    // control
    For,
    If,
    Else,
    Match,

    // type keywords are nested within the child enum
    Type(TypeKeyword),
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Ok(type_keyword) = TypeKeyword::from_str(string) {
            return Ok(Keyword::Type(type_keyword));
        }

        match string {
            "inputs" => Ok(Keyword::Inputs),
            "witness" => Ok(Keyword::Witness),
            "require" => Ok(Keyword::Require),

            "let" => Ok(Keyword::Let),
            "mut" => Ok(Keyword::Mut),

            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "match" => Ok(Keyword::Match),

            other => Err(other.to_string()),
        }
    }
}
