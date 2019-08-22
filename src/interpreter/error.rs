//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::FieldError;
use crate::lexical::Location;

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(display = "{} field: {}", _0, _1)]
    Field(Location, FieldError),
}
