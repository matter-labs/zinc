//!
//! The expression object.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
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
