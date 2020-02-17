//!
//! The function-local statement.
//!

use crate::syntax::ConstStatement;
use crate::syntax::Expression;
use crate::syntax::LetStatement;
use crate::syntax::LoopStatement;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Loop(LoopStatement),
    Empty,
    Expression(Expression),
}
