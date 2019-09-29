//!
//! The operator expression object.
//!

use std::fmt;

use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Operator(OperatorExpressionOperator),
    Operand(OperatorExpressionOperand),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operator(operator) => write!(f, "{}", operator),
            Self::Operand(operand) => write!(f, "{}", operand),
        }
    }
}
