//!
//! The expression object.
//!

use crate::syntax::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Operator(ExpressionOperator),
    Operand(ExpressionOperand),
    Auxiliary(ExpressionAuxiliary),
}
