//!
//! The type.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::ExpressionOperator;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum Type {
    Void,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
    Bool,
    //    Struct(Identifier, Vec<(Identifier, Type)>),
    //    Enum(Identifier, Vec<Identifier>),
    //    Tuple(Vec<Type>),
    //    MemoryVector(Box<Type>, usize),
    //    StorageVector(Box<Type>, usize),
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        true
    }

    pub fn can_be_first_operand(&self, operator: ExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Uint { .. }, ExpressionOperator::Addition) => true,
            (Self::Uint { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, ExpressionOperator::Division) => true,
            (Self::Uint { .. }, ExpressionOperator::Remainder) => true,
            (Self::Uint { .. }, ExpressionOperator::Negation) => true,

            (Self::Int { .. }, ExpressionOperator::Addition) => true,
            (Self::Int { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, ExpressionOperator::Division) => true,
            (Self::Int { .. }, ExpressionOperator::Remainder) => true,
            (Self::Int { .. }, ExpressionOperator::Negation) => true,

            (Self::Field { .. }, ExpressionOperator::Addition) => true,
            (Self::Field { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Field { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Field { .. }, ExpressionOperator::Division) => true,
            (Self::Field { .. }, ExpressionOperator::Remainder) => true,
            (Self::Field { .. }, ExpressionOperator::Negation) => true,

            _ => false,
        }
    }

    pub fn can_be_second_operand(&self, operator: ExpressionOperator) -> bool {
        match (self, operator) {
            (Self::Uint { .. }, ExpressionOperator::Addition) => true,
            (Self::Uint { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Uint { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Uint { .. }, ExpressionOperator::Division) => true,
            (Self::Uint { .. }, ExpressionOperator::Remainder) => true,

            (Self::Int { .. }, ExpressionOperator::Addition) => true,
            (Self::Int { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Int { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Int { .. }, ExpressionOperator::Division) => true,
            (Self::Int { .. }, ExpressionOperator::Remainder) => true,

            (Self::Field { .. }, ExpressionOperator::Addition) => true,
            (Self::Field { .. }, ExpressionOperator::Subtraction) => true,
            (Self::Field { .. }, ExpressionOperator::Multiplication) => true,
            (Self::Field { .. }, ExpressionOperator::Division) => true,
            (Self::Field { .. }, ExpressionOperator::Remainder) => true,

            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Void => write!(f, "()"),
            Type::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Type::Int { bitlength } => write!(f, "int{}", bitlength),
            Type::Field => write!(f, "field"),
            Type::Bool => write!(f, "bool"),
        }
    }
}
