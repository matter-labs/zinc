//!
//! The semantic analyzer `impl` statement error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer `impl` statement error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Only structure or enumeration types can have an implementation, but another type was found.
    ExpectedStructureOrEnumeration {
        /// The invalid type location in the code.
        location: Location,
        /// The invalid type identifier.
        found: String,
    },
}
