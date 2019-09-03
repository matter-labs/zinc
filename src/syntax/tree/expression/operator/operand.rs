//!
//! The expression operand.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Literal;
use crate::syntax::BlockExpression;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operand {
    Literal(Literal),
    Identifier(Identifier),
    Type(Type),
    Block(BlockExpression),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::Block(block) => write!(f, "{}", block),
        }
    }
}
