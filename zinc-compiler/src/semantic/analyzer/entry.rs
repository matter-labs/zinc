//!
//! The entry point semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::Tree;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::analyzer::statement::Context as StatementAnalyzerContext;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::Tree as SyntaxTree;

///
/// Analyzes the project entry, which must be located in the `main.zn` file.
///
/// To analyze a project module, use the module analyzer.
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

    pub fn compile(
        self,
        program: SyntaxTree,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<Tree, CompilerError> {
        let mut intermediate = Tree::new();

        let mut analyzer = StatementAnalyzer::new(self.scope_stack.top(), dependencies);
        for statement in program.statements.into_iter() {
            if let Some(statement) = analyzer
                .local_mod(statement, StatementAnalyzerContext::Entry)
                .map_err(CompilerError::Semantic)?
            {
                intermediate.statements.push(statement);
            }
        }

        let main_function_location = self.scope_stack.top().borrow().get_main_location();
        let contract_location = self.scope_stack.top().borrow().get_contract_location();

        if main_function_location.is_none() && contract_location.is_none() {
            return Err(CompilerError::Semantic(Error::EntryPointMissing));
        }

        if let (Some(main_location), Some(contract_location)) =
            (main_function_location, contract_location)
        {
            return Err(CompilerError::Semantic(Error::EntryPointAmbiguous {
                main: main_location,
                contract: contract_location,
            }));
        }

        Ok(intermediate)
    }
}
