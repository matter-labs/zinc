//!
//! The semantic analyzer `impl` statement error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedNamespace { location: Location, found: String },
}
