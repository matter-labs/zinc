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
}
