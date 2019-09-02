//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::StackElement;
use crate::lexical::Location;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Error {
    #[fail(display = "{} operator: {}", _0, _1)]
    Operator(Location, OperatorError),
    #[fail(display = "{} undeclared variable: {}", _0, _1)]
    UndeclaredVariable(Location, String),
    #[fail(display = "{} require failure: {}", _0, _1)]
    RequireFailure(Location, String),
}

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum OperatorError {
    #[fail(
        display = "operator {} is not available for the first operand: [ {} ]",
        operator, element
    )]
    FirstOperandOperatorNotAvailable {
        operator: ExpressionOperator,
        element: StackElement,
    },
    #[fail(
        display = "operator {} is not available for the second operand: [ {} ]",
        operator, element
    )]
    SecondOperandOperatorNotAvaiable {
        operator: ExpressionOperator,
        element: StackElement,
    },
    #[fail(display = "operand type mismatch: got {}, expected {}", got, expected)]
    OperandTypesMismatch {
        operator: ExpressionOperator,
        got: TypeVariant,
        expected: TypeVariant,
    },
    #[fail(
        display = "type expression allowed only as the second casting operand, but got {} for operator {}",
        element, operator
    )]
    TypeExpressionNotAsTheCastingOperand {
        operator: ExpressionOperator,
        element: StackElement,
    },
    #[fail(display = "assigning to a value expression: {} to {}", rvalue, lvalue)]
    AssignmentToValueExpression {
        lvalue: StackElement,
        rvalue: StackElement,
    },
    #[fail(display = "assigning type expression: {} to {}", rvalue, lvalue)]
    AssignmentTypeExpression {
        lvalue: StackElement,
        rvalue: StackElement,
    },
    #[fail(display = "assigning to a type expression: {} to {}", rvalue, lvalue)]
    AssignmentToTypeExpression {
        lvalue: StackElement,
        rvalue: StackElement,
    },
    #[fail(display = "casting to a place expression: {} to {}", rvalue, lvalue)]
    CastingToPlaceExpression {
        lvalue: StackElement,
        rvalue: StackElement,
    },
    #[fail(display = "casting to a value expression: {} to {}", rvalue, lvalue)]
    CastingToValueExpression {
        lvalue: StackElement,
        rvalue: StackElement,
    },
    #[fail(display = "casting to lesser bitlength: {} to {}", from, to)]
    CastingToLesserBitlength { from: TypeVariant, to: TypeVariant },
    #[fail(display = "casting invalid types: {} to {}", from, to)]
    CastingInvalidTypes { from: TypeVariant, to: TypeVariant },
}

impl OperatorError {
    pub fn first_operand_operator_not_available(
        operator: ExpressionOperator,
        element: StackElement,
    ) -> Self {
        Self::FirstOperandOperatorNotAvailable { operator, element }
    }

    pub fn second_operand_operator_not_available(
        operator: ExpressionOperator,
        element: StackElement,
    ) -> Self {
        Self::SecondOperandOperatorNotAvaiable { operator, element }
    }

    pub fn operand_type_mismatch(
        operator: ExpressionOperator,
        got: TypeVariant,
        expected: TypeVariant,
    ) -> Self {
        Self::OperandTypesMismatch {
            operator,
            got,
            expected,
        }
    }

    pub fn type_expression_not_as_the_casting_operand(
        operator: ExpressionOperator,
        element: StackElement,
    ) -> Self {
        Self::TypeExpressionNotAsTheCastingOperand { operator, element }
    }

    pub fn assignment_to_value_expression(lvalue: StackElement, rvalue: StackElement) -> Self {
        Self::AssignmentToValueExpression { lvalue, rvalue }
    }

    pub fn assignment_type_expression(lvalue: StackElement, rvalue: StackElement) -> Self {
        Self::AssignmentTypeExpression { lvalue, rvalue }
    }

    pub fn assignment_to_type_expression(lvalue: StackElement, rvalue: StackElement) -> Self {
        Self::AssignmentToTypeExpression { lvalue, rvalue }
    }

    pub fn casting_to_value_expression(lvalue: StackElement, rvalue: StackElement) -> Self {
        Self::CastingToValueExpression { lvalue, rvalue }
    }

    pub fn casting_to_place_expression(lvalue: StackElement, rvalue: StackElement) -> Self {
        Self::CastingToPlaceExpression { lvalue, rvalue }
    }

    pub fn casting_to_lesser_bitlength(from: TypeVariant, to: TypeVariant) -> Self {
        Self::CastingToLesserBitlength { from, to }
    }

    pub fn casting_invalid_types(from: TypeVariant, to: TypeVariant) -> Self {
        Self::CastingInvalidTypes { from, to }
    }
}
