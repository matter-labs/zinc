//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::ArrayError;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::InferenceError;
use crate::semantic::ScopeError;
use crate::semantic::StructureError;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),

    #[fail(display = "{} array literal: {}", _0, _1)]
    ArrayLiteral(Location, ArrayError),
    #[fail(display = "{} structure literal: {}", _0, _1)]
    StructureLiteral(Location, StructureError),

    #[fail(display = "{} constant type inference: {}", _0, _1)]
    ConstantTypeInference(Location, InferenceError),
    #[fail(display = "{} loop bounds type inference: {}", _0, _1)]
    LoopBoundsTypeInference(Location, InferenceError),

    #[fail(display = "{} let expected type, but got '{}'", _0, _1)]
    ExpectedType(Location, Element),

    #[fail(
        display = "{} conditional expected a boolean expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanExpression(Location, Type),
    #[fail(
        display = "{} conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypeMismatch(Location, Type, Type),

    #[fail(display = "{} calling a not-callable object '{}'", _0, _1)]
    FunctionCallOnNotCallable(Location, Element),
    #[fail(
        display = "{} function {} expected {} arguments, but got {}",
        _0, _1, _2, _3
    )]
    FunctionArgumentCountMismatch(Location, usize, usize, usize),
    #[fail(
        display = "{} function {} argument '{}' type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3, _4
    )]
    FunctionArgumentTypeMismatch(Location, usize, String, Type, Type),
    #[fail(
        display = "{} function {} return type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3
    )]
    FunctionReturnTypeMismatch(Location, usize, Type, Type),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,
    #[fail(display = "{} instruction function {} is unknown", _0, _1)]
    FunctionInstructionUnknown(Location, usize),
}
