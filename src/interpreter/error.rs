//!
//! The interpreter error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::Field;
use crate::lexical::Location;
use crate::syntax::ExpressionOperator;
use crate::syntax::Type;

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(
        display = "{} operator {} is not available for the first operand: [ {} ]",
        location, operator, field
    )]
    FirstOperandOperatorNotAvailable {
        location: Location,
        operator: ExpressionOperator,
        field: Field,
    },
    #[fail(
        display = "{} operator {} is not available for the second operand: [ {} ]",
        location, operator, field
    )]
    SecondOperandOperatorNotAvaiable {
        location: Location,
        operator: ExpressionOperator,
        field: Field,
    },
    #[fail(
        display = "{} operand type mismatch: got {}, expected {}",
        location, got, expected
    )]
    OperandTypesMismatch {
        location: Location,
        got: Type,
        expected: Type,
    },
}

impl Error {
    pub fn first_operand_operator_not_available(
        location: Location,
        operator: ExpressionOperator,
        field: Field,
    ) -> Self {
        Self::FirstOperandOperatorNotAvailable {
            location,
            operator,
            field,
        }
    }

    pub fn second_operand_operator_not_available(
        location: Location,
        operator: ExpressionOperator,
        field: Field,
    ) -> Self {
        Self::SecondOperandOperatorNotAvaiable {
            location,
            operator,
            field,
        }
    }

    pub fn operand_type_mismatch(location: Location, got: Type, expected: Type) -> Self {
        Self::OperandTypesMismatch {
            location,
            got,
            expected,
        }
    }
}
