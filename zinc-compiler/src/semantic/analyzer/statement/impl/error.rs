//!
//! The semantic analyzer `impl` statement error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedStructureOrEnumeration { location: Location, found: String },
}
