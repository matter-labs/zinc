//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::standard::error::Error as StandardFunctionError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::scope::error::Error as ScopeError;

#[derive(Debug, Fail, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{}: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{}: {}", _0, _1)]
    Scope(Location, ScopeError),

    #[fail(display = "{}: constant type inference: {}", _0, _1)]
    InferenceConstant(Location, IntegerConstantError),

    #[fail(display = "{}: array literal: {}", _0, _1)]
    LiteralArray(Location, ArrayValueError),
    #[fail(display = "{}: structure literal: {}", _0, _1)]
    LiteralStructure(Location, StructureValueError),

    #[fail(display = "{}: match expression must be exhausted", _0)]
    MatchNotExhausted(Location),
    #[fail(display = "{}: match branch is unreachable", _0)]
    MatchBranchUnreachable(Location),
    #[fail(
        display = "{}: match path pattern path must be resolved to an evaluable, but got '{}'",
        _0, _1
    )]
    MatchBranchPatternPathExpectedEvaluable(Location, String),
    #[fail(
        display = "{}: match pattern type '{}' does not match the scrutinee type '{}'",
        _0, _1, _2
    )]
    MatchBranchPatternInvalidType(Location, String, String),
    #[fail(
        display = "{}: match expression type '{}' does not match the first branch result type '{}'",
        _0, _1, _2
    )]
    MatchBranchExpressionInvalidType(Location, String, String),

    #[fail(
        display = "{}: cannot assign a value of type '{}' to a variable of type '{}'",
        _0, _1, _2
    )]
    AssignmentTypesMismatch(Location, String, String),
    #[fail(display = "{}: cannot assign to an immutable variable '{}'", _0, _1)]
    AssignmentToImmutableMemory(Location, String),

    #[fail(
        display = "{}: loop expected a boolean expression in the while condition, but got '{}'",
        _0, _1
    )]
    LoopWhileExpectedBooleanCondition(Location, String),
    #[fail(
        display = "{}: loop expected a range expression as bounds, but got '{}'",
        _0, _1
    )]
    LoopBoundsExpectedConstantRangeExpression(Location, String),

    #[fail(
        display = "{}: conditional expected a boolean condition expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanCondition(Location, String),
    #[fail(
        display = "{}: conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypesMismatch(Location, String, String),

    #[fail(display = "{}: calling a non-callable object '{}'", _0, _1)]
    FunctionCallingNotCallableObject(Location, String),
    #[fail(
        display = "{}: function '{}' expected {} arguments, but got {}",
        _0, _1, _2, _3
    )]
    FunctionArgumentCountMismatch(Location, String, usize, usize),
    #[fail(
        display = "{}: function '{}' argument '{}' type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3, _4
    )]
    FunctionArgumentTypeMismatch(Location, String, String, String, String),
    #[fail(
        display = "{}: function '{}' return type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3
    )]
    FunctionReturnTypeMismatch(Location, String, String, String),
    #[fail(display = "{}: built-in function '{}' is unknown", _0, _1)]
    FunctionInstructionUnknown(Location, String),
    #[fail(display = "{}: built-in function '{}' must be called with '!'", _0, _1)]
    FunctionInstructionSpecifierMissing(Location, &'static str),
    #[fail(
        display = "{}: function '{}' expected a constant unsigned length argument, but got '{}'",
        _0, _1, _2
    )]
    FunctionExpectedConstantLengthArgument(Location, &'static str, String),

    #[fail(display = "{}: {}", _0, _1)]
    FunctionStandardLibrary(Location, StandardFunctionError),
    #[fail(display = "{}: {}", _0, _1)]
    FunctionBuiltIn(Location, BuiltInFunctionError),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,

    #[fail(display = "{}: module '{}' not found in the project", _0, _1)]
    ModuleNotFound(Location, String),

    #[fail(
        display = "{}: use statement expected a path to an item, but got '{}'",
        _0, _1
    )]
    UseExpectedPath(Location, String),

    #[fail(
        display = "{}: only structures and enumerations can have implementations '{}'",
        _0, _1
    )]
    ImplStatementExpectedStructureOrEnumeration(Location, String),

    #[fail(
        display = "{}: the type alias does not point to type, but to '{}'",
        _0, _1
    )]
    TypeAliasDoesNotPointToType(Location, String),
    #[fail(
        display = "{}: the type alias does not point to structure, but to '{}'",
        _0, _1
    )]
    TypeAliasDoesNotPointToStructure(Location, String),
    #[fail(
        display = "{}: constant expression contains a non-constant element '{}'",
        _0, _1
    )]
    ConstantExpressionHasNonConstantElement(Location, String),
}
