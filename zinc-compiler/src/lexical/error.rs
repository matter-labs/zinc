//!
//! The lexical error.
//!

use failure::Fail;

use crate::lexical::stream::integer::Error as IntegerParserError;
use crate::lexical::stream::symbol::Error as SymbolParserError;
use crate::lexical::stream::word::Error as WordParserError;
use crate::lexical::token::location::Location;

#[derive(Debug, Fail, Clone, PartialEq)]
pub enum Error {
    #[fail(display = "{} invalid symbol: {}", _0, _1)]
    InvalidSymbol(Location, SymbolParserError),
    #[fail(display = "{} invalid word: {}", _0, _1)]
    InvalidWord(Location, WordParserError),
    #[fail(display = "{} invalid integer literal: {}", _0, _1)]
    InvalidInteger(Location, IntegerParserError),
    #[fail(display = "{} unexpected end of a token", _0)]
    UnexpectedEnd(Location),
}
