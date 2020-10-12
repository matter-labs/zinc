//!
//! The module.
//!

use crate::tree::statement::local_mod::Statement as ModuleLocalStatement;

///
/// The module, which is contained in a single file and consists of several module-level statements.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    /// The module statements.
    pub statements: Vec<ModuleLocalStatement>,
}

impl Module {
    ///
    /// Creates a module with statements.
    ///
    pub fn new(statements: Vec<ModuleLocalStatement>) -> Self {
        Self { statements }
    }
}
