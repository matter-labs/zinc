//!
//! The interpreter warning.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::executor::ScopeWarning;
use crate::lexical::Location;

#[derive(Debug, Fail, Serialize)]
pub enum Warning {
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeWarning),
}
