//!
//! The expression tree node.
//!

pub mod operand;
pub mod operator;

use self::operand::Operand;
use self::operator::Operator;

///
/// The expression tree node.
///
/// Operators are branches, operands are leaves.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Operator(Operator),
    Operand(Operand),
}

impl Node {
    pub fn operator(operator: Operator) -> Self {
        Self::Operator(operator)
    }

    pub fn operand(operand: Operand) -> Self {
        Self::Operand(operand)
    }
}
