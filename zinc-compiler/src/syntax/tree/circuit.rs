//!
//! The syntax tree.
//!

use crate::syntax::ModuleLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxTree {
    pub statements: Vec<ModuleLocalStatement>,
}
