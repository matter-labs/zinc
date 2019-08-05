//!
//! The keyword lexeme.
//!

use std::str::FromStr;

use failure::Fail;

#[derive(Debug)]
pub enum Delimiter {
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    BracketRoundOpen,
    BracketRoundClose,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown")]
    Unknown,
}

impl FromStr for Delimiter {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "{" => Ok(Delimiter::BracketCurlyOpen),
            "}" => Ok(Delimiter::BracketCurlyClose),
            "[" => Ok(Delimiter::BracketSquareOpen),
            "]" => Ok(Delimiter::BracketSquareClose),
            "(" => Ok(Delimiter::BracketRoundOpen),
            ")" => Ok(Delimiter::BracketRoundClose),

            _unknown => Err(Error::Unknown),
        }
    }
}
