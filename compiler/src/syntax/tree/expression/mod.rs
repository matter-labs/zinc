//!
//! The expression.
//!

mod array;
mod block;
mod conditional;
mod operator;

pub use self::array::Builder as ArrayExpressionBuilder;
pub use self::array::Expression as ArrayExpression;
pub use self::block::Builder as BlockExpressionBuilder;
pub use self::block::Expression as BlockExpression;
pub use self::conditional::Builder as ConditionalExpressionBuilder;
pub use self::conditional::Expression as ConditionalExpression;
pub use self::operator::Builder as OperatorExpressionBuilder;
pub use self::operator::Element as OperatorExpressionElement;
pub use self::operator::Expression as OperatorExpression;
pub use self::operator::Object as OperatorExpressionObject;
pub use self::operator::Operand as OperatorExpressionOperand;
pub use self::operator::Operator as OperatorExpressionOperator;

use std::fmt;

use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Operator(OperatorExpression),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
}

impl Expression {
    pub fn location(&self) -> Location {
        match self {
            Self::Operator(expression) => expression.location,
            Self::Block(expression) => expression.location,
            Self::Conditional(expression) => expression.location,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operator(expression) => write!(f, "( {} )", expression),
            Self::Block(expression) => write!(f, "{}", expression),
            Self::Conditional(expression) => write!(f, "{}", expression),
        }
    }
}
