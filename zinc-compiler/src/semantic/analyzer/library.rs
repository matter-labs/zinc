//!
//! The library semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::bytecode::Bytecode;
use crate::semantic::scope::Scope;
use crate::SyntaxTree;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    bytecode: Rc<RefCell<Bytecode>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(Bytecode::new())))
    }
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;

    pub fn new(bytecode: Rc<RefCell<Bytecode>>) -> Self {
        Self {
            scope_stack: {
                let mut scopes = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scopes.push(Rc::new(RefCell::new(Scope::new_global())));
                scopes
            },
            bytecode,
        }
    }

    pub fn compile(self, program: SyntaxTree) -> Result<Rc<RefCell<Scope>>, CompilerError> {
        self.bytecode.borrow_mut().push_data_stack_address();
        for statement in program.statements.into_iter() {
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), HashMap::new())
                .module_local_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }
        self.bytecode.borrow_mut().pop_data_stack_address();

        Ok(self.scope())
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }
}
