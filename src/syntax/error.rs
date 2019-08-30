//!
//! The syntax error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Lexeme;
use crate::lexical::Location;

#[derive(Debug, Fail, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "{} expected either of: {:?} (got {})", _0, _1, _2)]
    Expected(Location, Vec<&'static str>, Lexeme),
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
}
