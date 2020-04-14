//!
//! The semantic analyzer conditional expression error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedBooleanCondition {
        location: Location,
        found: String,
    },
    BranchTypesMismatch {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },
}
