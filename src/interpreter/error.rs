//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::lexical::Location;
use crate::syntax::ExpressionOperand;
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
        operator, value
    )]
    FirstOperandOperatorNotAvailable {
        operator: ExpressionOperator,
        value: Value,
    },
    #[fail(
        display = "operator {} is not available for the second operand: [ {} ]",
        operator, value
    )]
    SecondOperandOperatorNotAvaiable {
        operator: ExpressionOperator,
        value: Value,
    },
    #[fail(display = "operand type mismatch: got {}, expected {}", got, expected)]
    OperandTypesMismatch {
        got: TypeVariant,
        expected: TypeVariant,
    },
    #[fail(display = "casting to lesser bitlength: {} to {}", from, to)]
    CastingToLesserBitlength { from: TypeVariant, to: TypeVariant },
}

impl OperatorError {
    pub fn first_operand_operator_not_available(
        operator: ExpressionOperator,
        value: Value,
    ) -> Self {
        Self::FirstOperandOperatorNotAvailable { operator, value }
    }

    pub fn second_operand_operator_not_available(
        operator: ExpressionOperator,
        value: Value,
    ) -> Self {
        Self::SecondOperandOperatorNotAvaiable { operator, value }
    }

    pub fn operand_type_mismatch(got: TypeVariant, expected: TypeVariant) -> Self {
        Self::OperandTypesMismatch { got, expected }
    }

    pub fn casting_to_lesser_bitlength(from: TypeVariant, to: TypeVariant) -> Self {
        Self::CastingToLesserBitlength { from, to }
    }
}
