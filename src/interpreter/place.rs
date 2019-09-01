//!
//! The interpreter place.
//!

use std::fmt;
use std::str;

use failure::Fail;
use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::interpreter::Value;
use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Place {
    identifier: Identifier,
    value: Value,
}

impl Place {
    pub fn new(identifier: Identifier, value: Value) -> Self {
        Self { identifier, value }
    }

    pub fn assign(&mut self, value: Value) -> Result<(), OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Assignment;

        //        if !self.value.type_variant.can_be_first_operand(OPERATOR) {
        //            return Err(OperatorError::first_operand_operator_not_available(
        //                OPERATOR, self,
        //            ));
        //        }
        if !value.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, value,
            ));
        }
        if self.value.type_variant != value.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                value.type_variant,
                self.value.type_variant,
            ));
        }

        self.value = value;
        Ok(())
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
