//!
//! The semantic analyzer value element error.
//!

use failure::Fail;

use crate::semantic::IntegerError;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "operator '{}' expected a unit value, but got '{}'", _0, _1)]
    ExpectedUnit(&'static str, Type),
    #[fail(
        display = "operator '{}' expected a boolean value, but got '{}'",
        _0, _1
    )]
    ExpectedBoolean(&'static str, Type),
    #[fail(
        display = "operator '{}' expected an integer value, but got '{}'",
        _0, _1
    )]
    ExpectedInteger(&'static str, Type),
    #[fail(
        display = "operator '{}' expected two primitive type values, but got '{}' and '{}'",
        _0, _1, _2
    )]
    ExpectedPrimitiveTypes(&'static str, Type, Type),
    #[fail(display = "integer: {}", _0)]
    Integer(IntegerError),
    #[fail(display = "assigning a value of type '{}' to '{}'", _0, _1)]
    AssignmentTypeMismatch(Type, Type),
}
