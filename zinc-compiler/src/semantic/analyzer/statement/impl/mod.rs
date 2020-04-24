//!
//! The `impl` statement semantic analyzer.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#fn::Context as FnStatementAnalyzerContext;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::Variant as ScopeItemVariant;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes an implementation statement and returns its IR for the next compiler phase.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: ImplStatement,
    ) -> Result<Vec<GeneratorStatement>, Error> {
        let identifier_location = statement.identifier.location;

        let mut scope_stack = ScopeStack::new(scope);

        let mut intermediate = Vec::new();

        let namespace_scope =
            match Scope::resolve_item(scope_stack.top(), &statement.identifier, true)?.variant {
                ScopeItemVariant::Type(Type::Structure(inner)) => inner.scope,
                ScopeItemVariant::Type(Type::Enumeration(inner)) => inner.scope,
                item => {
                    return Err(Error::Statement(StatementError::Impl(
                        ImplStatementError::ExpectedStructureOrEnumeration {
                            location: identifier_location,
                            found: item.to_string(),
                        },
                    )));
                }
            };

        scope_stack.push_scope(namespace_scope);
        let mut analyzer = StatementAnalyzer::new(scope_stack.top(), HashMap::new());
        for statement in statement.statements.into_iter() {
            if let Some(statement) =
                analyzer.local_impl(statement, FnStatementAnalyzerContext::Implementation)?
            {
                intermediate.push(statement);
            }
        }
        scope_stack.pop();

        Ok(intermediate)
    }
}
