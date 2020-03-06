//!
//! The semantic error.
//!

use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::scope::error::Error as ScopeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Element(Location, ElementError),
    Scope(Location, ScopeError),
    Function(Location, FunctionError),

    MatchNotExhausted {
        location: Location,
    },
    MatchBranchUnreachable {
        location: Location,
    },
    MatchBranchPatternPathExpectedEvaluable {
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

    MutatingWithDifferentType {
        location: Location,
        expected: String,
        found: String,
    },
    MutatingImmutableMemory {
        location: Location,
        name: String,
        reference: Option<Location>,
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

    StructureDuplicateField {
        location: Location,
        type_identifier: String,
        field_name: String,
    },

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

    TypeAliasDoesNotPointToType {
        location: Location,
        found: String,
    },
    TypeAliasDoesNotPointToStructure {
        location: Location,
        found: String,
    },
    ConstantExpressionHasNonConstantElement {
        location: Location,
        found: String,
    },
}
