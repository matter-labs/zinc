//!
//! The expression object.
//!

use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Operator(ExpressionOperator),
    Operand(ExpressionOperand),
}
