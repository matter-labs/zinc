//!
//! The binary semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::semantic::Bytecode;
use crate::semantic::Error;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::StatementAnalyzer;
use crate::semantic::Type;
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

    pub fn compile(
        self,
        program: SyntaxTree,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<(), CompilerError> {
        // compile all the outer statements which generally only declare new items
        let mut analyzer =
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), dependencies);
        for statement in program.statements.into_iter() {
            analyzer
                .module_local_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        // replace the placeholders inserted above with an actual 'main' function call
        let main_function_address = self
            .bytecode
            .borrow_mut()
            .function_address("main")
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;

        if let Ok(ScopeItem::Type(Type::Function {
            arguments,
            return_type,
            ..
        })) = Scope::resolve_item(self.scope(), "main")
        {
            let input_size = arguments.iter().map(|(_name, r#type)| r#type.size()).sum();
            let output_size = return_type.size();

            self.bytecode.borrow_mut().set_input_fields(arguments);
            self.bytecode.borrow_mut().set_output_type(*return_type);
            self.bytecode.borrow_mut().set_main_function(
                main_function_address,
                input_size,
                output_size,
            );
        }

        Ok(())
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }
}
