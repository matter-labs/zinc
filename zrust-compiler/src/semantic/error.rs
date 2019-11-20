//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::InferenceError;
use crate::semantic::ScopeError;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(display = "{} inference: {}", _0, _1)]
    LoopBoundsTypeInference(Location, InferenceError),

    #[fail(
        display = "{} conditional expected a boolean expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanExpression(Location, TypeVariant),
    #[fail(
        display = "{} conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypeMismatch(Location, TypeVariant, TypeVariant),

    #[fail(display = "{} calling a not-callable object '{}'", _0, _1)]
    FunctionCallOnNotCallable(Location, Element),
    #[fail(
        display = "{} function '{}' expected {} arguments, but got {}",
        _0, _1, _2, _3
    )]
    FunctionArgumentCountMismatch(Location, String, usize, usize),
    #[fail(
        display = "{} function '{}' argument '{}' type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3, _4
    )]
    FunctionArgumentTypeMismatch(Location, String, String, TypeVariant, TypeVariant),
    #[fail(
        display = "{} function '{}' return type mismatch: expected '{}', but got '{}'",
        _0, _1, _2, _3
    )]
    FunctionReturnTypeMismatch(Location, String, TypeVariant, TypeVariant),
    #[fail(display = "function 'main' is missing")]
    FunctionMainMissing,
}
