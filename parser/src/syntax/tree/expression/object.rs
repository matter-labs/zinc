//!
//! The expression object.
//!

use std::fmt;

use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Operator(ExpressionOperator),
    Operand(ExpressionOperand),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operator(operator) => write!(f, "{}", operator),
            Self::Operand(operand) => write!(f, "{}", operand),
        }
    }
}
