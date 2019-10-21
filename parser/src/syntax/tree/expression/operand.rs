//!
//! The expression operand.
//!

use std::fmt;

use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;
use crate::syntax::MatchExpression;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Unit,
    Literal(Literal),
    Identifier(Identifier),
    Type(Type),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    Structure(StructureExpression),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Literal(operand) => write!(f, "{}", operand),
            Self::Identifier(operand) => write!(f, "{}", operand),
            Self::Type(operand) => write!(f, "{}", operand),
            Self::Block(operand) => write!(f, "{}", operand),
            Self::Conditional(operand) => write!(f, "{}", operand),
            Self::Match(operand) => write!(f, "{}", operand),
            Self::Array(operand) => write!(f, "{}", operand),
            Self::Tuple(operand) => write!(f, "{}", operand),
            Self::Structure(operand) => write!(f, "{}", operand),
        }
    }
}
