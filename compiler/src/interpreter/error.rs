//!
//! The interpreter error.
//!

use failure::Fail;

use crate::interpreter::ElementError;
use crate::interpreter::ScopeError;
use crate::interpreter::Value;
use crate::lexical::Location;
use crate::syntax::Literal;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{} synthesis: {}", _0, _1)]
    Synthesis(Location, String),
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(display = "{} literal is not supported: {}", _0, _1)]
    LiteralIsNotSupported(Location, Literal),
    #[fail(
        display = "{} the require {} expected a boolean expression, but got [{}]",
        _0, _1, _2
    )]
    RequireExpectedBooleanExpression(Location, String, Value),
    #[fail(display = "{} the require {} failed", _0, _1)]
    RequireFailed(Location, String),
    #[fail(
        display = "{} let declaration invalid type: [{}] cannot be casted to '{}'",
        _0, _1, _2
    )]
    LetDeclarationInvalidType(Location, Value, TypeVariant),
    #[fail(
        display = "{} invalid loop range: the start [{}] is greater than the end [{}]",
        _0, _1, _2
    )]
    LoopRangeInvalid(Location, Value, Value),
    #[fail(
        display = "{} conditional expected a boolean expression, but got [{}]",
        _0, _1
    )]
    ConditionalExpectedBooleanExpression(Location, Value),
}
