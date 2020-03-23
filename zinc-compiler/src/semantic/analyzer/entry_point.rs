//!
//! The entry point semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::Representation;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::error::Error;
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
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<Representation, CompilerError> {
        let mut intermediate = Representation::new();

        // compile all the outer statements which generally only declare new items
        let mut analyzer = StatementAnalyzer::new(self.scope(), dependencies);
        for statement in program.statements.into_iter() {
            if let Some(statement) = analyzer
                .module_local_statement(statement)
                .map_err(CompilerError::Semantic)?
            {
                intermediate.statements.push(statement);
            }
        }

        Scope::resolve_item(self.scope(), "main")
            .map_err(|_| Error::EntryPointMissing)
            .map_err(CompilerError::Semantic)?;

        Ok(intermediate)
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::semantic::Error as SemanticError;

    #[test]
    fn test() {
        let input = r#"
fn another() -> u8 {
    42
}
"#;

        let expected = Err(Error::Semantic(SemanticError::EntryPointMissing));

        let result = crate::semantic::tests::compile_entry_point(input);

        assert_eq!(result, expected);
    }
}
