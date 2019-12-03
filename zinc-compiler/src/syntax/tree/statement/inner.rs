//!
//! The inner statement.
//!

use crate::syntax::Expression;
use crate::syntax::LetStatement;
use crate::syntax::LoopStatement;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Empty,
    Let(LetStatement),
    Loop(LoopStatement),
    Expression(Expression),
}
