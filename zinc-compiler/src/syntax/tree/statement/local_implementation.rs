//!
//! The implementation-local statement.
//!

use crate::syntax::ConstStatement;
use crate::syntax::FnStatement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Const(ConstStatement),
    Fn(FnStatement),
}
