//!
//! The expression semantic analyzer stack element.
//!

use crate::semantic::element::Element as SemanticElement;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;

#[derive(Debug, Clone)]
pub enum Element {
    NotEvaluated(ExpressionOperand),
    Evaluated(SemanticElement),
}
