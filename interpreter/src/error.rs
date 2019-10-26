//!
//! The interpreter error.
//!

use failure::Fail;

use parser::Location;
use parser::TypeVariant;
use semantic::InferenceError as SemanticError;

use crate::element::Error as ElementError;
use crate::element::IntegerError;
use crate::element::Value;
use crate::scope::Error as ScopeError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(display = "{} semantic: {}", _0, _1)]
    Semantic(Location, SemanticError),
    #[fail(
        display = "{} the require '{}' expected a boolean expression, but got '{}'",
        _0, _1, _2
    )]
    RequireExpectedBooleanExpression(Location, String, Value),
    #[fail(display = "{} the require '{}' failed", _0, _1)]
    RequireFailed(Location, String),
    #[fail(
        display = "{} let declaration invalid type: '{}' cannot be casted to '{}'",
        _0, _1, _2
    )]
    LetInvalidType(Location, TypeVariant, TypeVariant),
    #[fail(display = "{} let declaration implicit casting: {}", _0, _1)]
    LetImplicitCasting(Location, IntegerError),
    #[fail(
        display = "{} loop while condition expected a boolean expression, but got '{}'",
        _0, _1
    )]
    LoopWhileExpectedBooleanExpression(Location, Value),
    #[fail(
        display = "{} conditional expected a boolean expression, but got '{}'",
        _0, _1
    )]
    ConditionalExpectedBooleanExpression(Location, Value),
    #[fail(
        display = "{} conditional branches return different types: '{}' and '{}'",
        _0, _1, _2
    )]
    ConditionalBranchTypeMismatch(Location, Value, Value),
    #[fail(display = "{} enumeration '{}' has no variant '{}'", _0, _1, _2)]
    EnumerationVariantNotExists(Location, String, String),

    #[fail(display = "calling a not-callable object '{}'", _0)]
    CallingNotCallable(String),
    #[fail(display = "call expected an argument list, but got '{}'", _0)]
    CallExpectedArgumentList(String),
}
