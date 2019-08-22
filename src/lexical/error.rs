//!
//! The lexical error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerParserError;
use crate::lexical::Location;
use crate::lexical::SymbolParserError;

#[derive(Debug, Fail, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "{} character '{}' is currently forbidden", _0, _1)]
    InvalidCharacter(Location, char),
    #[fail(display = "{} invalid symbol: {}", _0, _1)]
    InvalidSymbol(Location, SymbolParserError),
    #[fail(display = "{} invalid integer literal: {}", _0, _1)]
    InvalidIntegerLiteral(Location, IntegerParserError),
}
