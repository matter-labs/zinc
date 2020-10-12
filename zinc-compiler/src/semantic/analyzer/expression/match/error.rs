//!
//! The semantic analyzer `match` expression error.
//!

use zinc_lexical::Location;

///
/// The semantic analyzer `match` expression error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Only primitive types can act as scrutinee types (be matched) for now.
    ScrutineeInvalidType {
        /// The error location data.
        location: Location,
        /// The invalid type, which is actually found.
        found: String,
    },
    /// The `match` patterns do not cover all the possible values of the scrutinee expression type.
    NotExhausted {
        /// The error location data.
        location: Location,
    },
    /// A `match` expression must have at least two branches to generate a useful conditional code.
    LessThanTwoBranches {
        /// The error location data.
        location: Location,
    },
    /// A branch with an refutable pattern appears after the irrefutable one, that is, after the
    /// branch, whose pattern always matches.
    BranchUnreachable {
        /// The error location data.
        location: Location,
    },
    /// Only constants can act as the branch patterns.
    BranchPatternPathExpectedConstant {
        /// The error location data.
        location: Location,
        /// The invalid expression, which is actually found.
        found: String,
    },
    /// A branch pattern type does not match the scrutinee expression type.
    BranchPatternInvalidType {
        /// The error location data.
        location: Location,
        /// The expected branch pattern type, which is dictated by the scrutinee expression type.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The another branch location, which helps user to find the error.
        reference: Location,
    },
    /// A subsequent branch result expression type does not match the first branch expression type.
    BranchExpressionInvalidType {
        /// The error location data.
        location: Location,
        /// The expected branch result type, which is dictated by the first branch result type.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The first branch location, which helps user to find the error.
        reference: Location,
    },
    /// Some branch pattern occurs more than once in the `match` expression.
    BranchDuplicate {
        /// The error location data.
        location: Location,
        /// The first branch location, which helps user to find the error.
        reference: Location,
    },
}
