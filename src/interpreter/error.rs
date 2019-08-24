//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::lexical::Location;

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(display = "{} operator: {}", _0, _1)]
    Operator(Location, OperatorError),
    #[fail(display = "{} undeclared variable: {}", _0, _1)]
    UndeclaredVariable(Location, String),
    #[fail(display = "redeclared variable: {}", _0)]
    RedeclaredVariable(String),
    #[fail(display = "require failure")]
    RequireFailure,
}
