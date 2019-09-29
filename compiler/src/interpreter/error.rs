//!
//! The interpreter error.
//!

use failure::Fail;

use crate::interpreter::ElementError;
use crate::interpreter::IntegerError;
use crate::interpreter::ScopeError;
use crate::interpreter::Value;
use crate::lexical::Location;
use crate::syntax::Literal;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(display = "{} literal cannot be evaluated: {}", _0, _1)]
    LiteralCannotBeEvaluated(Location, Literal),
    #[fail(
        display = "{} conditional expected a boolean expression, but got [{}]",
        _0, _1
    )]
    ConditionalExpectedBooleanExpression(Location, Value),
    #[fail(
        display = "{} let declaration invalid type: [{}] cannot be casted to '{}'",
        _0, _1, _2
    )]
    LetInvalidType(Location, Value, TypeVariant),
    #[fail(display = "{} let declaration invalid implicit casting: {}", _0, _1)]
    LetImplicitCasting(Location, IntegerError),
    #[fail(
        display = "{} the require '{}' expected a boolean expression, but got [{}]",
        _0, _1, _2
    )]
    RequireExpectedBooleanExpression(Location, String, Value),
    #[fail(display = "{} the require '{}' failed", _0, _1)]
    RequireFailed(Location, String),
    #[fail(display = "{} loop iterator: {}", _0, _1)]
    LoopIterator(Location, IntegerError),
}
