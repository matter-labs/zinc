//!
//! The semantic analyzer error.
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

    MatchNotExhausted(Location),
    MatchBranchUnreachable(Location),
    MatchBranchPatternPathExpectedEvaluable(Location, String),
    MatchBranchPatternInvalidType(Location, String, String, Location),
    MatchBranchExpressionInvalidType(Location, String, String, Location),

    MutatingWithDifferentType(Location, String, String),
    MutatingImmutableMemory(Location, String, Option<Location>),

    LoopWhileExpectedBooleanCondition(Location, String),
    LoopBoundsExpectedConstantRangeExpression(Location, String),

    ConditionalExpectedBooleanCondition(Location, String),
    ConditionalBranchTypesMismatch(Location, String, String, Location),

    EntryPointMissing,

    StructureDuplicateField(Location, String, String),

    ModuleNotFound(Location, String),

    UseExpectedPath(Location, String),

    ImplStatementExpectedStructureOrEnumeration(Location, String),

    TypeAliasDoesNotPointToType(Location, String),
    TypeAliasDoesNotPointToStructure(Location, String),
    ConstantExpressionHasNonConstantElement(Location, String),
}
