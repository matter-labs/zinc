//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::ArrayValueError;
use crate::semantic::ElementError;
use crate::semantic::IntegerConstantError;
use crate::semantic::ScopeError;
use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::StructureValueError;

#[derive(Debug, Fail, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),

    #[fail(display = "{} constant type inference: {}", _0, _1)]
    InferenceConstant(Location, IntegerConstantError),

    #[fail(display = "{} array literal: {}", _0, _1)]
    LiteralArray(Location, ArrayValueError),
    #[fail(display = "{} structure literal: {}", _0, _1)]
    LiteralStructure(Location, StructureValueError),

    #[fail(display = "{} match expression must be exhausted", _0)]
    MatchNotExhausted(Location),
    #[fail(display = "{} match branch is unreachable", _0)]
    MatchBranchUnreachable(Location),
    #[fail(
        display = "{} match path pattern path must be resolved to an evaluable, but got '{}'",
        _0, _1
    )]
    MatchBranchPatternPathExpectedEvaluable(Location, String),
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

    #[fail(
        display = "{} assigning a value of type '{}' to a variable of type '{}'",
        _0, _1, _2
    )]
    AssignmentTypesMismatch(Location, String, String),
    #[fail(display = "{} assigning to an immutable path '{}'", _0, _1)]
    AssignmentToImmutableMemory(Location, String),

    #[fail(
        display = "{} loop expected a boolean expression in the while condition, but got '{}'",
        _0, _1
    )]
    LoopWhileExpectedBooleanCondition(Location, String),
    #[fail(
        display = "{} loop expected a range expression as bounds, but got '{}'",
        _0, _1
    )]
    LoopBoundsExpectedConstantRangeExpression(Location, String),

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
    #[fail(display = "{} built-in function '{}' is unknown", _0, _1)]
    FunctionInstructionUnknown(Location, String),
    #[fail(display = "{} built-in function '{}' must be called with '!'", _0, _1)]
    FunctionInstructionSpecifierMissing(Location, &'static str),
    #[fail(
        display = "{} function '{}' expected a constant argument, but got '{}'",
        _0, _1, _2
    )]
    FunctionExpectedConstantArgument(Location, &'static str, String),

    #[fail(display = "{} {}", _0, _1)]
    FunctionStandardLibrary(Location, StandardLibraryFunctionError),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,

    #[fail(display = "{} module '{}' not found in the project", _0, _1)]
    ModuleNotFound(Location, String),

    #[fail(
        display = "{} use statement expected a path expression, but got '{}'",
        _0, _1
    )]
    UseExpectedPath(Location, String),

    #[fail(
        display = "{} impl statement expected a structure or enumeration, but got '{}'",
        _0, _1
    )]
    ImplStatementExpectedStructureOrEnumeration(Location, String),

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
    #[fail(
        display = "{} instruction 'assert' expected a boolean, but got '{}'",
        _0, _1
    )]
    InstructionAssertExpectedBoolean(Location, String),

    #[fail(display = "references are not implemented yet")]
    ReferencesNotImplemented,
}
