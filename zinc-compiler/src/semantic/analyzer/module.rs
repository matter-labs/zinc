//!
//! The module semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::Representation;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::scope::Scope;
use crate::syntax::Tree as SyntaxTree;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            scope_stack: {
                let mut scopes = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scopes.push(Rc::new(RefCell::new(Scope::new_global())));
                scopes
            },
        }
    }

    pub fn compile(
        self,
        program: SyntaxTree,
    ) -> Result<(Rc<RefCell<Scope>>, Representation), CompilerError> {
        let mut intermediate = Representation::new();

        // self.bytecode.borrow_mut().push_data_stack_address();
        let mut analyzer = StatementAnalyzer::new(self.scope(), HashMap::new());
        for statement in program.statements.into_iter() {
            if let Some(statement) = analyzer
                .module_local_statement(statement)
                .map_err(CompilerError::Semantic)?
            {
                intermediate.statements.push(statement);
            }
        }
        // self.bytecode.borrow_mut().pop_data_stack_address();

        Ok((self.scope(), intermediate))
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }
}
