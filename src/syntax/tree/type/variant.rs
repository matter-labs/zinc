//!
//! The type variant.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum Variant {
    Void,
    Bool,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
}

impl Variant {
    pub fn uint(bitlength: usize) -> Self {
        Self::Uint { bitlength }
    }

    pub fn int(bitlength: usize) -> Self {
        Self::Int { bitlength }
    }

    pub fn can_be_first_operand(&self, operator: OperatorExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Void, OperatorExpressionOperator::Assignment) => true,

            (Self::Bool, OperatorExpressionOperator::Assignment) => true,
            (Self::Bool, OperatorExpressionOperator::Or) => true,
            (Self::Bool, OperatorExpressionOperator::Xor) => true,
            (Self::Bool, OperatorExpressionOperator::And) => true,
            (Self::Bool, OperatorExpressionOperator::Equal) => true,
            (Self::Bool, OperatorExpressionOperator::NotEqual) => true,
            (Self::Bool, OperatorExpressionOperator::Not) => true,

            (Self::Uint { .. }, OperatorExpressionOperator::Assignment) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Equal) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::NotEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Greater) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Lesser) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Addition) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Division) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Remainder) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Casting) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Negation) => true,

            (Self::Int { .. }, OperatorExpressionOperator::Assignment) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Equal) => true,
            (Self::Int { .. }, OperatorExpressionOperator::NotEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Greater) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Lesser) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Addition) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Division) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Remainder) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Casting) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Negation) => true,

            (Self::Field, OperatorExpressionOperator::Assignment) => true,
            (Self::Field, OperatorExpressionOperator::Equal) => true,
            (Self::Field, OperatorExpressionOperator::NotEqual) => true,
            (Self::Field, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Field, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Field, OperatorExpressionOperator::Greater) => true,
            (Self::Field, OperatorExpressionOperator::Lesser) => true,
            (Self::Field, OperatorExpressionOperator::Addition) => true,
            (Self::Field, OperatorExpressionOperator::Subtraction) => true,
            (Self::Field, OperatorExpressionOperator::Multiplication) => true,
            (Self::Field, OperatorExpressionOperator::Division) => true,
            (Self::Field, OperatorExpressionOperator::Remainder) => true,
            (Self::Field, OperatorExpressionOperator::Negation) => true,

            _ => false,
        }
    }

    pub fn can_be_second_operand(&self, operator: OperatorExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Void, OperatorExpressionOperator::Assignment) => true,

            (Self::Bool, OperatorExpressionOperator::Assignment) => true,
            (Self::Bool, OperatorExpressionOperator::Or) => true,
            (Self::Bool, OperatorExpressionOperator::Xor) => true,
            (Self::Bool, OperatorExpressionOperator::And) => true,
            (Self::Bool, OperatorExpressionOperator::Equal) => true,
            (Self::Bool, OperatorExpressionOperator::NotEqual) => true,

            (Self::Uint { .. }, OperatorExpressionOperator::Assignment) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Equal) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::NotEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Greater) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Lesser) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Addition) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Division) => true,
            (Self::Uint { .. }, OperatorExpressionOperator::Remainder) => true,

            (Self::Int { .. }, OperatorExpressionOperator::Assignment) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Equal) => true,
            (Self::Int { .. }, OperatorExpressionOperator::NotEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Greater) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Lesser) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Addition) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Division) => true,
            (Self::Int { .. }, OperatorExpressionOperator::Remainder) => true,

            (Self::Field, OperatorExpressionOperator::Assignment) => true,
            (Self::Field, OperatorExpressionOperator::Equal) => true,
            (Self::Field, OperatorExpressionOperator::NotEqual) => true,
            (Self::Field, OperatorExpressionOperator::GreaterEqual) => true,
            (Self::Field, OperatorExpressionOperator::LesserEqual) => true,
            (Self::Field, OperatorExpressionOperator::Greater) => true,
            (Self::Field, OperatorExpressionOperator::Lesser) => true,
            (Self::Field, OperatorExpressionOperator::Addition) => true,
            (Self::Field, OperatorExpressionOperator::Subtraction) => true,
            (Self::Field, OperatorExpressionOperator::Multiplication) => true,
            (Self::Field, OperatorExpressionOperator::Division) => true,
            (Self::Field, OperatorExpressionOperator::Remainder) => true,

            _ => false,
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),
        }
    }
}
