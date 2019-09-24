//!
//! The interpreter element.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::Element;
use crate::interpreter::ValueError;
use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Error {
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
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
