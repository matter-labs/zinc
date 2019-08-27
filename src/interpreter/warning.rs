//!
//! The interpreter warning.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Location;

#[derive(Debug, Fail, Serialize)]
pub enum Warning {
    #[fail(display = "{} redeclared variable: {}", _0, _1)]
    RedeclaredVariable(Location, String),
}
