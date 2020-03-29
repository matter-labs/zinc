//!
//! The function-local statement.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;
use crate::syntax::tree::statement::r#loop::Statement as LoopStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Loop(LoopStatement),
    Empty(Location),
    Expression(ExpressionTree),
}

impl Statement {
    pub fn location(&self) -> Location {
        match self {
            Self::Let(inner) => inner.location,
            Self::Const(inner) => inner.location,
            Self::Loop(inner) => inner.location,
            Self::Empty(location) => *location,
            Self::Expression(inner) => inner.location,
        }
    }
}
