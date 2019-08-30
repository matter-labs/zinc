//!
//! The lexical error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerParserError;
use crate::lexical::Location;
use crate::lexical::SymbolParserError;
use crate::lexical::WordParserError;

#[derive(Debug, Fail, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end of input when parsing token at {}", _0)]
    UnexpectedEnd(Location),
    #[fail(display = "{} character '{}' is unknown yet", _0, _1)]
    UnknownCharacter(Location, char),
    #[fail(display = "{} invalid symbol: {}", _0, _1)]
    InvalidSymbol(Location, SymbolParserError),
    #[fail(display = "{} invalid word: {}", _0, _1)]
    InvalidWord(Location, WordParserError),
    #[fail(display = "{} invalid integer literal: {}", _0, _1)]
    InvalidIntegerLiteral(Location, IntegerParserError),
}
