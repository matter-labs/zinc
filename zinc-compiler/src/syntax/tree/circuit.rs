//!
//! The syntax tree.
//!

use crate::syntax::OuterStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub statements: Vec<OuterStatement>,
}
