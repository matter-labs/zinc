//!
//! The module semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::Tree;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::Tree as SyntaxTree;

///
/// Analyzes a module, which are located in non-`main.zn` files.
///
/// To analyze the project entry, use the entry analyzer.
///
pub struct Analyzer {
    scope_stack: ScopeStack,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scope_stack: ScopeStack::new_global(),
        }
    }

    pub fn compile(self, program: SyntaxTree) -> Result<(Rc<RefCell<Scope>>, Tree), CompilerError> {
        let mut intermediate = Tree::new();

        let mut analyzer = StatementAnalyzer::new(self.scope_stack.top(), HashMap::new());
        for statement in program.statements.into_iter() {
            if let Some(statement) = analyzer
                .local_mod(statement)
                .map_err(CompilerError::Semantic)?
            {
                intermediate.statements.push(statement);
            }
        }

        Ok((self.scope_stack.top(), intermediate))
    }
}
