//!
//! The semantic analyzer element error.
//!

use failure::Fail;

use crate::semantic::Element;
use crate::semantic::PlaceError;
use crate::semantic::ScopeError;
use crate::semantic::ValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
    #[fail(display = "place: {}", _0)]
    Place(PlaceError),
    #[fail(display = "scope: {}", _0)]
    Scope(ScopeError),
    #[fail(
        display = "operator '{}' expected a place expression, but got '{}'",
        _0, _1
    )]
    ExpectedPlaceExpression(&'static str, Element),
    #[fail(
        display = "operator '{}' expected a value expression, but got '{}'",
        _0, _1
    )]
    ExpectedValueExpression(&'static str, Element),
    #[fail(
        display = "operator '{}' expected a value or place expression, but got '{}'",
        _0, _1
    )]
    ExpectedValueOrPlaceExpression(&'static str, Element),
    #[fail(
        display = "operator '{}' expected a type expression, but got '{}'",
        _0, _1
    )]
    ExpectedTypeExpression(&'static str, Element),
}
