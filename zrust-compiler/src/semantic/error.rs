//!
//! The semantic analyzer error.
//!

use failure::Fail;

use crate::lexical::Location;
use crate::semantic::ElementError;
use crate::semantic::InferenceError;
use crate::semantic::IntegerError;
use crate::semantic::ScopeError;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} element: {}", _0, _1)]
    Element(Location, ElementError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(
        display = "{} let declaration invalid type: '{}' cannot be casted to '{}'",
        _0, _1, _2
    )]
    LetInvalidType(Location, TypeVariant, TypeVariant),
    #[fail(display = "{} let declaration implicit semantic.casting: {}", _0, _1)]
    LetImplicitCasting(Location, IntegerError),
    #[fail(display = "{} inference: {}", _0, _1)]
    Inference(Location, InferenceError),
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
}
