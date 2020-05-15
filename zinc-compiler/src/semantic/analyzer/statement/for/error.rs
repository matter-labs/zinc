//!
//! The semantic analyzer `for` statement error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    WhileExpectedBooleanCondition { location: Location, found: String },
    BoundsExpectedConstantRangeExpression { location: Location, found: String },
}
