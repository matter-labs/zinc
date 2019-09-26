//!
//! The interpreter element.
//!

use failure::Fail;

use crate::interpreter::Element;
use crate::interpreter::IntegerError;
use crate::interpreter::Value;
use crate::interpreter::ValueError;
use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "synthesis: {}", _0)]
    Synthesis(String),
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
    #[fail(display = "integer: {}", _0)]
    Integer(IntegerError),
    #[fail(display = "comparing invalid values: [{}] and [{}]", _0, _1)]
    ComparingInvalidValues(Value, Value),
    #[fail(
        display = "operator '{}' expected a boolean value, but got [{}]",
        _0, _1
    )]
    ExpectedBooleanValue(OperatorExpressionOperator, Element),
    #[fail(
        display = "operator '{}' expected an integer value, but got [{}]",
        _0, _1
    )]
    ExpectedIntegerValue(OperatorExpressionOperator, Element),
    #[fail(
        display = "operator '{}' expected a place expression, but got [{}]",
        _0, _1
    )]
    ExpectedPlaceExpression(OperatorExpressionOperator, Element),
    #[fail(
        display = "operator '{}' expected a value expression, but got [{}]",
        _0, _1
    )]
    ExpectedValueExpression(OperatorExpressionOperator, Element),
    #[fail(
        display = "operator '{}' expected a type expression, but got [{}]",
        _0, _1
    )]
    ExpectedTypeExpression(OperatorExpressionOperator, Element),
}
