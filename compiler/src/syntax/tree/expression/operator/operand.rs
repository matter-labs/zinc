//!
//! The expression operand.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::Type;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operand {
    Literal(Literal),
    Identifier(Identifier),
    Type(Type),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(operand) => write!(f, "{}", operand),
            Self::Identifier(operand) => write!(f, "{}", operand),
            Self::Type(operand) => write!(f, "{}", operand),
            Self::Block(operand) => write!(f, "{}", operand),
            Self::Conditional(operand) => write!(f, "{}", operand),
        }
    }
}
