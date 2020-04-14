//!
//! The semantic analyzer `match` expression error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ScrutineeInvalidType {
        location: Location,
        found: String,
    },
    NotExhausted {
        location: Location,
    },
    LessThanTwoBranches {
        location: Location,
    },
    BranchUnreachable {
        location: Location,
    },
    BranchPatternPathExpectedConstant {
        location: Location,
        found: String,
    },
    BranchPatternInvalidType {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },
    BranchExpressionInvalidType {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },
    BranchDuplicate {
        location: Location,
        reference: Location,
    },
}
