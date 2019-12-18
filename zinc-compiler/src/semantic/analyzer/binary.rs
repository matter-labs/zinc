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
                scopes.push(Rc::new(RefCell::new(Scope::default())));
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
        self.scope()
            .borrow_mut()
            .declare_type(
                "dbg".to_owned(),
                Type::new_function(
                    "dbg".to_owned(),
                    vec![("format".to_owned(), Type::String)],
                    Type::Unit,
                ),
            )
            .expect(crate::semantic::PANIC_FUNCTION_INSTRUCTION_DECLARATION);

        self.scope()
            .borrow_mut()
            .declare_type(
                "assert".to_owned(),
                Type::new_function(
                    "assert".to_owned(),
                    vec![
                        ("condition".to_owned(), Type::Boolean),
                        ("message".to_owned(), Type::String),
                    ],
                    Type::Unit,
                ),
            )
            .expect(crate::semantic::PANIC_FUNCTION_INSTRUCTION_DECLARATION);

        // compile all the outer statements which generally only declare new items
        let mut analyzer =
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), dependencies);
        for statement in program.statements.into_iter() {
            analyzer
                .outer_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        // replace the placeholders inserted above with an actual 'main' function call
        let main_function_address = self
            .bytecode
            .borrow_mut()
            .function_address("main")
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;
        match Scope::resolve_item(self.scope(), "main")
            .expect(crate::semantic::PANIC_FUNCTION_RESOLUTION_MAIN)
        {
            ScopeItem::Type(Type::Function {
                arguments,
                return_type,
                ..
            }) => {
                let input_size = arguments
                    .into_iter()
                    .map(|(_arg_name, arg_type)| arg_type.size())
                    .sum();
                let output_size = return_type.size();

                self.bytecode.borrow_mut().set_main_function(
                    main_function_address,
                    input_size,
                    output_size,
                );
            }
            _ => panic!(crate::semantic::PANIC_FUNCTION_RESOLUTION_MAIN),
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
