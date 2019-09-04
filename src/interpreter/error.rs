//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::Element;
use crate::interpreter::IntegerType;
use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Fail, Serialize, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[fail(display = "{} operator: {}", _0, _1)]
    Operator(Location, OperatorError),
    #[fail(display = "{} undeclared variable: {}", _0, _1)]
    UndeclaredVariable(Location, String),
    #[fail(display = "{} the literal is not supported: {}", _0, _1)]
    LiteralIsNotSupported(Location, Literal),
    #[fail(
        display = "{} require {} expected a boolean expression, but got {}",
        _0, _1, _2
    )]
    RequireExpectedBooleanExpression(Location, String, Value),
    #[fail(display = "{} require {} failed", _0, _1)]
    RequireFailed(Location, String),
}

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum OperatorError {
    #[fail(
        display = "operand types mismatch: [{}] and [{}] have different types",
        first, second
    )]
    OperandTypesMismatch {
        operator: OperatorExpressionOperator,
        first: Element,
        second: Element,
    },
    #[fail(
        display = "operator {} expected a boolean value, but got [{}]",
        operator, got
    )]
    ExpectedBooleanValue {
        operator: OperatorExpressionOperator,
        got: Element,
    },
    #[fail(
        display = "operator {} expected an integer value, but got [{}]",
        operator, got
    )]
    ExpectedIntegerValue {
        operator: OperatorExpressionOperator,
        got: Element,
    },
    #[fail(display = "operator {} expected a type, but got [{}]", operator, got)]
    ExpectedType {
        operator: OperatorExpressionOperator,
        got: Element,
    },
    #[fail(
        display = "operator {} expected a place expression, but got [{}]",
        operator, lvalue
    )]
    ExpectedPlaceExpression {
        operator: OperatorExpressionOperator,
        lvalue: Element,
    },
    #[fail(
        display = "operator {} expected a value expression, but got [{}]",
        rvalue, rvalue
    )]
    ExpectedValueExpression {
        operator: OperatorExpressionOperator,
        rvalue: Element,
    },
    #[fail(
        display = "assignment to an immutable variable: [{}] to [{}]",
        rvalue, lvalue
    )]
    AssignmentToImmutableVariable { lvalue: Place, rvalue: Element },
    #[fail(display = "casting to lesser bitlength: [{}] to [{}]", from, to)]
    CastingToLesserBitlength { from: IntegerType, to: IntegerType },
}

impl OperatorError {
    pub fn operand_type_mismatch(
        operator: OperatorExpressionOperator,
        first: Element,
        second: Element,
    ) -> Self {
        Self::OperandTypesMismatch {
            operator,
            first,
            second,
        }
    }

    pub fn expected_boolean_value(operator: OperatorExpressionOperator, got: Element) -> Self {
        Self::ExpectedBooleanValue { operator, got }
    }

    pub fn expected_integer_value(operator: OperatorExpressionOperator, got: Element) -> Self {
        Self::ExpectedIntegerValue { operator, got }
    }

    pub fn expected_type(operator: OperatorExpressionOperator, got: Element) -> Self {
        Self::ExpectedType { operator, got }
    }

    pub fn expected_place_expression(
        operator: OperatorExpressionOperator,
        lvalue: Element,
    ) -> Self {
        Self::ExpectedPlaceExpression { operator, lvalue }
    }

    pub fn expected_value_expression(
        operator: OperatorExpressionOperator,
        rvalue: Element,
    ) -> Self {
        Self::ExpectedValueExpression { operator, rvalue }
    }

    pub fn assignment_to_immutable_variable(lvalue: Place, rvalue: Element) -> Self {
        Self::AssignmentToImmutableVariable { lvalue, rvalue }
    }

    pub fn casting_to_lesser_bitlength(from: IntegerType, to: IntegerType) -> Self {
        Self::CastingToLesserBitlength { from, to }
    }
}
