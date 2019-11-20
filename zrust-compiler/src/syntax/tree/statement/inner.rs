//!
//! The inner statement.
//!

use std::fmt;

use crate::syntax::Expression;
use crate::syntax::LetStatement;
use crate::syntax::LoopStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Empty,
    Let(LetStatement),
    Loop(LoopStatement),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ";"),
            Self::Let(statement) => write!(f, "{}", statement),
            Self::Loop(statement) => write!(f, "{}", statement),
            Self::Expression(statement) => write!(f, "{}", statement),
        }
    }
}
