//!
//! The semantic analyzer element value error.
//!

use failure::Fail;

use crate::semantic::IntegerError;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "operator '{}' expected a unit value, but got '{}'", _0, _1)]
    ExpectedUnit(&'static str, TypeVariant),
    #[fail(
        display = "operator '{}' expected a boolean value, but got '{}'",
        _0, _1
    )]
    ExpectedBoolean(&'static str, TypeVariant),
    #[fail(
        display = "operator '{}' expected an integer value, but got '{}'",
        _0, _1
    )]
    ExpectedInteger(&'static str, TypeVariant),
    #[fail(display = "integer: {}", _0)]
    Integer(IntegerError),
    #[fail(display = "assigning a value of type '{}' to '{}'", _0, _1)]
    AssignmentTypeMismatch(TypeVariant, TypeVariant),
}
