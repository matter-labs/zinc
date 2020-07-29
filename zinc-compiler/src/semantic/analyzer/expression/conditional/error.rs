//!
//! The semantic analyzer conditional expression error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer conditional expression error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The condition is not of boolean type.
    ExpectedBooleanCondition {
        /// The error location data.
        location: Location,
        /// The invalid condition type, which is actually found.
        found: String,
    },
    /// The conditional branches must return the same type, but it is not so.
    BranchTypesMismatch {
        /// The error location data.
        location: Location,
        /// The expected type, which is dictated by the first (main- or then-) branch result.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The another branch location, which helps user to find the error.
        reference: Location,
    },
}
