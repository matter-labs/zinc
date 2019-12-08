//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::ArrayError;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::IntegerConstantError;
use crate::semantic::Place;
use crate::semantic::ScopeError;
use crate::semantic::StructureError;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),

    #[fail(display = "{} array literal: {}", _0, _1)]
    LiteralArray(Location, ArrayError),
    #[fail(display = "{} structure literal: {}", _0, _1)]
    LiteralStructure(Location, StructureError),

    #[fail(display = "{} constant type inference: {}", _0, _1)]
    TypeInferenceConstant(Location, IntegerConstantError),
    #[fail(display = "{} loop bounds type inference: {}", _0, _1)]
    TypeInferenceLoopBounds(Location, IntegerConstantError),
    #[fail(display = "{} match pattern type inference: {}", _0, _1)]
    TypeInferenceMatchPattern(Location, IntegerConstantError),

    #[fail(display = "{} match expression must be exhausted", _0)]
    MatchNotExhausted(Location),
    #[fail(display = "{} match branch is unreachable", _0)]
    MatchBranchUnreachable(Location),
    #[fail(
        display = "{} match pattern type '{}' does not match the scrutinee type '{}'",
        _0, _1, _2
    )]
    MatchBranchPatternInvalidType(Location, Type, Type),
    #[fail(
        display = "{} match expression type '{}' does not match the scrutinee type '{}'",
        _0, _1, _2
    )]
    MatchBranchExpressionInvalidType(Location, Type, Type),

    #[fail(display = "{} expected a type, but got '{}'", _0, _1)]
    ExpectedType(Location, Element),
    #[fail(display = "{} expected a value, but got '{}'", _0, _1)]
    ExpectedValue(Location, Element),

    #[fail(
        display = "{} assigning a value of type '{}' to a memory place '{}' of type '{}'",
        _0, _1, _2, _3
    )]
    AssignmentTypesMismatch(Location, Type, Place, Type),
    #[fail(display = "{} assigning to an immutable memory place '{}'", _0, _1)]
    AssignmentToImmutableMemory(Location, Place),

    #[fail(
        display = "{} conditional expected a boolean condition expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanCondition(Location, Type),
    #[fail(
        display = "{} conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypesMismatch(Location, Type, Type),

    #[fail(display = "{} calling a non-callable object '{}'", _0, _1)]
    FunctionCallNotCallableObject(Location, Element),
    #[fail(
        display = "{} function '{}' expected {} arguments, but got {}",
        _0, _1, _2, _3
    )]
    FunctionArgumentCountMismatch(Location, String, usize, usize),
    #[fail(
        display = "{} function '{}' argument '{}' type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3, _4
    )]
    FunctionArgumentTypeMismatch(Location, String, String, Type, Type),
    #[fail(
        display = "{} function '{}' return type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3
    )]
    FunctionReturnTypeMismatch(Location, String, Type, Type),
    #[fail(display = "{} instruction function '{}' is unknown", _0, _1)]
    FunctionNotInstruction(Location, String),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,
    #[fail(
        display = "{} instruction 'dbg' expected a string, but got '{}'",
        _0, _1
    )]
    InstructionDebugExpectedString(Location, Element),

    #[fail(
        display = "{} operator '::' expected a namespace identifier, but got '{}'",
        _0, _1
    )]
    PathOperatorFirstOperandExpectedNamespace(Location, Element),
    #[fail(
        display = "{} operator '::' expected a namespace identifier, but got '{}'",
        _0, _1
    )]
    PathOperatorSecondOperandExpectedStringConstant(Location, Element),
}
