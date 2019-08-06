//!
//! The lexical error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerAnalyzerError;

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "character '{}' is currently forbidden", _0)]
    Forbidden(char),
    #[fail(display = "character is not allowed in the integer literal: {}", _0)]
    InvalidIntegerLiteral(IntegerAnalyzerError),
}
