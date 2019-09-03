//!
//! The expression.
//!

mod block;
mod operator;

pub use self::block::Expression as BlockExpression;
pub use self::operator::Element as OperatorExpressionElement;
pub use self::operator::Expression as OperatorExpression;
pub use self::operator::Object as OperatorExpressionObject;
pub use self::operator::Operand as OperatorExpressionOperand;
pub use self::operator::Operator as OperatorExpressionOperator;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;

#[derive(Debug, Serialize, PartialEq)]
pub enum Expression {
    Operator(OperatorExpression),
    Block(BlockExpression),
}

impl Expression {
    pub fn location(&self) -> Location {
        match self {
            Self::Operator(expression) => expression.location(),
            Self::Block(expression) => expression.location,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operator(expression) => write!(f, "{}", expression),
            Self::Block(expression) => write!(f, "{}", expression),
        }
    }
}
