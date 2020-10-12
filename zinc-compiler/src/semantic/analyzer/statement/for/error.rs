//!
//! The semantic analyzer `for` statement error.
//!

use zinc_lexical::Location;

///
/// The semantic analyzer `for` statement error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `while` condition is not of boolean type.
    WhileExpectedBooleanCondition {
        /// The condition expression location.
        location: Location,
        /// The stringified invalid condition type.
        found: String,
    },
    /// The loop bounds is not a constant range expression.
    BoundsExpectedConstantRangeExpression {
        /// The loop bounds expression location.
        location: Location,
        /// The stringified invalid bounds element.
        found: String,
    },
}
