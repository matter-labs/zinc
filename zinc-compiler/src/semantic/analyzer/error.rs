//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::ArrayError;
use crate::semantic::ElementError;
use crate::semantic::IntegerConstantError;
use crate::semantic::Place;
use crate::semantic::ScopeError;
use crate::semantic::StructureError;

#[derive(Debug, Fail, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),

    #[fail(display = "{} constant type inference: {}", _0, _1)]
    InferenceConstant(Location, IntegerConstantError),
    #[fail(display = "{} match pattern type inference: {}", _0, _1)]
    InferencePatternMatch(Location, IntegerConstantError),

    #[fail(display = "{} array literal: {}", _0, _1)]
    LiteralArray(Location, ArrayError),
    #[fail(display = "{} structure literal: {}", _0, _1)]
    LiteralStructure(Location, StructureError),

    #[fail(display = "{} match expression must be exhausted", _0)]
    MatchNotExhausted(Location),
    #[fail(display = "{} match branch is unreachable", _0)]
    MatchBranchUnreachable(Location),
    #[fail(
        display = "{} match pattern type '{}' does not match the scrutinee type '{}'",
        _0, _1, _2
    )]
    MatchBranchPatternInvalidType(Location, String, String),
    #[fail(
        display = "{} match expression type '{}' does not match the first branch result type '{}'",
        _0, _1, _2
    )]
    MatchBranchExpressionInvalidType(Location, String, String),

    #[fail(display = "{} assigning to an invalid item '{}'", _0, _1)]
    AssignmentToInvalidItem(Location, String),
    #[fail(
        display = "{} assigning a value of type '{}' to a memory place '{}' of type '{}'",
        _0, _1, _2, _3
    )]
    AssignmentTypesMismatch(Location, String, Place, String),
    #[fail(display = "{} assigning to an immutable memory place '{}'", _0, _1)]
    AssignmentToImmutableMemory(Location, Place),

    #[fail(
        display = "{} loop expected a boolean expression in the while condition, but got '{}'",
        _0, _1
    )]
    LoopWhileExpectedBooleanCondition(Location, String),
    #[fail(
        display = "{} loop expected an integer constant as the range start, but got '{}'",
        _0, _1
    )]
    LoopRangeStartExpectedIntegerConstant(Location, String),
    #[fail(
        display = "{} loop expected an integer constant as the range end, but got '{}'",
        _0, _1
    )]
    LoopRangeEndExpectedIntegerConstant(Location, String),

    #[fail(
        display = "{} conditional expected a boolean condition expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanCondition(Location, String),
    #[fail(
        display = "{} conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypesMismatch(Location, String, String),

    #[fail(display = "{} calling a non-callable object '{}'", _0, _1)]
    FunctionCallingNotCallableObject(Location, String),
    #[fail(
        display = "{} function '{}' expected {} arguments, but got {}",
        _0, _1, _2, _3
    )]
    FunctionArgumentCountMismatch(Location, String, usize, usize),
    #[fail(
        display = "{} function '{}' argument '{}' type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3, _4
    )]
    FunctionArgumentTypeMismatch(Location, String, String, String, String),
    #[fail(
        display = "{} function '{}' return type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3
    )]
    FunctionReturnTypeMismatch(Location, String, String, String),
    #[fail(display = "{} instruction function '{}' is unknown", _0, _1)]
    FunctionNotInstruction(Location, String),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,

    #[fail(
        display = "{} operator '::' expected a namespace identifier, but got '{}'",
        _0, _1
    )]
    PathOperatorFirstOperandExpectedNamespace(Location, String),
    #[fail(
        display = "{} operator '::' expected a namespace identifier, but got '{}'",
        _0, _1
    )]
    PathOperatorSecondOperandExpectedStringConstant(Location, String),

    #[fail(display = "{} module '{}' not found in the project", _0, _1)]
    ModuleNotFound(Location, String),

    #[fail(display = "{} the type alias does not point to type, but '{}'", _0, _1)]
    TypeAliasDoesNotPointToType(Location, String),
    #[fail(
        display = "{} the type alias does not point to structure, but '{}'",
        _0, _1
    )]
    TypeAliasDoesNotPointToStructure(Location, String),
    #[fail(
        display = "{} constant expression contains a non-constant element '{}'",
        _0, _1
    )]
    ConstantExpressionHasNonConstantElement(Location, String),
    #[fail(
        display = "{} instruction 'dbg' expected a string, but got '{}'",
        _0, _1
    )]
    InstructionDebugExpectedString(Location, String),
}
