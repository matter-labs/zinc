//!
//! The expression tree node.
//!

pub mod operand;
pub mod operator;

use std::fmt;

use self::operand::Operand;
use self::operator::Operator;

///
/// The expression tree node.
///
/// Operators are branches, operands are leaves.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// The operator node variant.
    Operator(Operator),
    /// The operand node variant.
    Operand(Operand),
}

impl Node {
    ///
    /// A shortcut constructor.
    ///
    pub fn operator(operator: Operator) -> Self {
        Self::Operator(operator)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn operand(operand: Operand) -> Self {
        Self::Operand(operand)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operator(inner) => write!(f, "{}", inner),
            Self::Operand(inner) => write!(f, "{}", inner),
        }
    }
}
