//!
//! The interpreter element error.
//!

use failure::Fail;

use crate::interpreter::Element;
use crate::interpreter::PlaceError;
use crate::interpreter::Value;
use crate::interpreter::ValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "synthesis: {}", _0)]
    Synthesis(String),
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
    #[fail(display = "place: {}", _0)]
    Place(PlaceError),
    #[fail(display = "comparing invalid values: [{}] and [{}]", _0, _1)]
    ComparingInvalidValues(Value, Value),
    #[fail(
        display = "operator '{}' expected a place expression, but got [{}]",
        _0, _1
    )]
    ExpectedPlaceExpression(&'static str, Element),
    #[fail(
        display = "operator '{}' expected a value expression, but got [{}]",
        _0, _1
    )]
    ExpectedValueExpression(&'static str, Element),
    #[fail(
        display = "operator '{}' expected a type expression, but got [{}]",
        _0, _1
    )]
    ExpectedTypeExpression(&'static str, Element),
}
