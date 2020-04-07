//!
//! The semantic error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::scope::error::Error as ScopeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Element(Location, ElementError),
    Scope(Location, ScopeError),

    MatchScrutineeInvalidType {
        location: Location,
        found: String,
    },
    MatchNotExhausted {
        location: Location,
    },
    MatchLessThanTwoBranches {
        location: Location,
    },
    MatchBranchUnreachable {
        location: Location,
    },
    MatchBranchPatternPathExpectedConstant {
        location: Location,
        found: String,
    },
    MatchBranchPatternInvalidType {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },
    MatchBranchExpressionInvalidType {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },
    MatchBranchDuplicate {
        location: Location,
        reference: Location,
    },

    LoopWhileExpectedBooleanCondition {
        location: Location,
        found: String,
    },
    LoopBoundsExpectedConstantRangeExpression {
        location: Location,
        found: String,
    },

    ConditionalExpectedBooleanCondition {
        location: Location,
        found: String,
    },
    ConditionalBranchTypesMismatch {
        location: Location,
        expected: String,
        found: String,
        reference: Location,
    },

    EntryPointMissing,

    ModuleNotFound {
        location: Location,
        name: String,
    },

    UseExpectedPath {
        location: Location,
        found: String,
    },

    ImplStatementExpectedStructureOrEnumeration {
        location: Location,
        found: String,
    },

    ConstantExpressionHasNonConstantElement {
        location: Location,
        found: String,
    },
}
