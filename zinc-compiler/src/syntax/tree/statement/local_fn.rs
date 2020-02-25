//!
//! The function-local statement.
//!

use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;
use crate::syntax::tree::statement::r#loop::Statement as LoopStatement;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Loop(LoopStatement),
    Empty,
    Expression(Expression),
}
