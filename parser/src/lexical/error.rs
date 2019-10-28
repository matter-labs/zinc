//!
//! The lexical error.
//!

use failure::Fail;

use crate::lexical::IntegerParserError;
use crate::lexical::Location;
use crate::lexical::SymbolParserError;
use crate::lexical::WordParserError;

#[derive(Debug, Fail, Clone, PartialEq)]
pub enum Error {
    #[fail(display = "{} unexpected end of the token", _0)]
    UnexpectedEnd(Location),
    #[fail(display = "{} the character '{}' is not yet valid", _0, _1)]
    InvalidCharacter(Location, char),
    #[fail(display = "{} invalid symbol: {}", _0, _1)]
    InvalidSymbol(Location, SymbolParserError),
    #[fail(display = "{} invalid word: {}", _0, _1)]
    InvalidWord(Location, WordParserError),
    #[fail(display = "{} invalid integer literal: {}", _0, _1)]
    InvalidIntegerLiteral(Location, IntegerParserError),
}
