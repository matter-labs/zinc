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
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::Tree as SyntaxTree;

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
    ) -> Result<Representation, CompilerError> {
        let mut intermediate = Representation::new();

        // compile all the outer statements which generally only declare new items
        let mut analyzer = StatementAnalyzer::new(self.scope_stack.top(), dependencies);
        for statement in program.statements.into_iter() {
            if let Some(statement) = analyzer
                .local_mod(statement)
                .map_err(CompilerError::Semantic)?
            {
                intermediate.statements.push(statement);
            }
        }

        Scope::resolve_item(
            self.scope_stack.top(),
            crate::semantic::element::r#type::function::user::FUNCTION_MAIN_IDENTIFIER,
        )
        .map_err(|_| Error::EntryPointMissing)
        .map_err(CompilerError::Semantic)?;

        Ok(intermediate)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::semantic::Error as SemanticError;

    #[test]
    fn error_test() {
        let input = r#"
fn another() -> u8 {
    42
}
"#;

        let expected = Err(Error::Semantic(SemanticError::EntryPointMissing));

        let result = crate::semantic::tests::compile_entry(input);

        assert_eq!(result, expected);
    }
}
