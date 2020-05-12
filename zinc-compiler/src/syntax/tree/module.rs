//!
//! The module.
//!

use crate::syntax::tree::statement::local_mod::Statement as ModuleLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub statements: Vec<ModuleLocalStatement>,
}

impl Module {
    pub fn new(statements: Vec<ModuleLocalStatement>) -> Self {
        Self { statements }
    }
}
