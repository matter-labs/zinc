//!
//! The implementation-local statement.
//!

use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Const(ConstStatement),
    Fn(FnStatement),
    Empty,
}
