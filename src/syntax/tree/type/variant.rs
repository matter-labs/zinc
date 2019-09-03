//!
//! The type variant.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::ExpressionOperator;

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

    pub fn can_be_first_operand(&self, operator: ExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Void, ExpressionOperator::Assignment) => true,

            (Self::Bool, ExpressionOperator::Assignment) => true,
            (Self::Bool, ExpressionOperator::Or) => true,
            (Self::Bool, ExpressionOperator::Xor) => true,
            (Self::Bool, ExpressionOperator::And) => true,
            (Self::Bool, ExpressionOperator::Equal) => true,
            (Self::Bool, ExpressionOperator::NotEqual) => true,
            (Self::Bool, ExpressionOperator::Not) => true,

            (Self::Uint { .. }, ExpressionOperator::Assignment) => true,
            (Self::Uint { .. }, ExpressionOperator::Equal) => true,
            (Self::Uint { .. }, ExpressionOperator::NotEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::GreaterEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::LesserEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::Greater) => true,
            (Self::Uint { .. }, ExpressionOperator::Lesser) => true,
            (Self::Uint { .. }, ExpressionOperator::Addition) => true,
            (Self::Uint { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, ExpressionOperator::Division) => true,
            (Self::Uint { .. }, ExpressionOperator::Remainder) => true,
            (Self::Uint { .. }, ExpressionOperator::Casting) => true,
            (Self::Uint { .. }, ExpressionOperator::Negation) => true,

            (Self::Int { .. }, ExpressionOperator::Assignment) => true,
            (Self::Int { .. }, ExpressionOperator::Equal) => true,
            (Self::Int { .. }, ExpressionOperator::NotEqual) => true,
            (Self::Int { .. }, ExpressionOperator::GreaterEqual) => true,
            (Self::Int { .. }, ExpressionOperator::LesserEqual) => true,
            (Self::Int { .. }, ExpressionOperator::Greater) => true,
            (Self::Int { .. }, ExpressionOperator::Lesser) => true,
            (Self::Int { .. }, ExpressionOperator::Addition) => true,
            (Self::Int { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, ExpressionOperator::Division) => true,
            (Self::Int { .. }, ExpressionOperator::Remainder) => true,
            (Self::Int { .. }, ExpressionOperator::Casting) => true,
            (Self::Int { .. }, ExpressionOperator::Negation) => true,

            (Self::Field, ExpressionOperator::Assignment) => true,
            (Self::Field, ExpressionOperator::Equal) => true,
            (Self::Field, ExpressionOperator::NotEqual) => true,
            (Self::Field, ExpressionOperator::GreaterEqual) => true,
            (Self::Field, ExpressionOperator::LesserEqual) => true,
            (Self::Field, ExpressionOperator::Greater) => true,
            (Self::Field, ExpressionOperator::Lesser) => true,
            (Self::Field, ExpressionOperator::Addition) => true,
            (Self::Field, ExpressionOperator::Subtraction) => true,
            (Self::Field, ExpressionOperator::Multiplication) => true,
            (Self::Field, ExpressionOperator::Division) => true,
            (Self::Field, ExpressionOperator::Remainder) => true,
            (Self::Field, ExpressionOperator::Negation) => true,

            _ => false,
        }
    }

    pub fn can_be_second_operand(&self, operator: ExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Void, ExpressionOperator::Assignment) => true,

            (Self::Bool, ExpressionOperator::Assignment) => true,
            (Self::Bool, ExpressionOperator::Or) => true,
            (Self::Bool, ExpressionOperator::Xor) => true,
            (Self::Bool, ExpressionOperator::And) => true,
            (Self::Bool, ExpressionOperator::Equal) => true,
            (Self::Bool, ExpressionOperator::NotEqual) => true,

            (Self::Uint { .. }, ExpressionOperator::Assignment) => true,
            (Self::Uint { .. }, ExpressionOperator::Equal) => true,
            (Self::Uint { .. }, ExpressionOperator::NotEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::GreaterEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::LesserEqual) => true,
            (Self::Uint { .. }, ExpressionOperator::Greater) => true,
            (Self::Uint { .. }, ExpressionOperator::Lesser) => true,
            (Self::Uint { .. }, ExpressionOperator::Addition) => true,
            (Self::Uint { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, ExpressionOperator::Division) => true,
            (Self::Uint { .. }, ExpressionOperator::Remainder) => true,

            (Self::Int { .. }, ExpressionOperator::Assignment) => true,
            (Self::Int { .. }, ExpressionOperator::Equal) => true,
            (Self::Int { .. }, ExpressionOperator::NotEqual) => true,
            (Self::Int { .. }, ExpressionOperator::GreaterEqual) => true,
            (Self::Int { .. }, ExpressionOperator::LesserEqual) => true,
            (Self::Int { .. }, ExpressionOperator::Greater) => true,
            (Self::Int { .. }, ExpressionOperator::Lesser) => true,
            (Self::Int { .. }, ExpressionOperator::Addition) => true,
            (Self::Int { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, ExpressionOperator::Division) => true,
            (Self::Int { .. }, ExpressionOperator::Remainder) => true,

            (Self::Field, ExpressionOperator::Assignment) => true,
            (Self::Field, ExpressionOperator::Equal) => true,
            (Self::Field, ExpressionOperator::NotEqual) => true,
            (Self::Field, ExpressionOperator::GreaterEqual) => true,
            (Self::Field, ExpressionOperator::LesserEqual) => true,
            (Self::Field, ExpressionOperator::Greater) => true,
            (Self::Field, ExpressionOperator::Lesser) => true,
            (Self::Field, ExpressionOperator::Addition) => true,
            (Self::Field, ExpressionOperator::Subtraction) => true,
            (Self::Field, ExpressionOperator::Multiplication) => true,
            (Self::Field, ExpressionOperator::Division) => true,
            (Self::Field, ExpressionOperator::Remainder) => true,

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
