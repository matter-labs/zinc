//!
//! The lexical error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerAnalyzerError;
use crate::lexical::Location;
use crate::lexical::OperatorAnalyzerError;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "{} character '{}' is currently forbidden", _0, _1)]
    Forbidden(Location, char),
    #[fail(display = "{} invalid integer literal: {}", _0, _1)]
    InvalidIntegerLiteral(Location, IntegerAnalyzerError),
    #[fail(display = "{} invalid operator: {}", _0, _1)]
    InvalidOperator(Location, OperatorAnalyzerError),
}
