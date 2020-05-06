//!
//! The `contract` statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only contract declaration statement.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: ContractStatement,
    ) -> Result<Vec<GeneratorStatement>, Error> {
        let location = statement.location;

        let mut scope_stack = ScopeStack::new(scope);

        let mut intermediate = Vec::new();

        scope_stack.push();
        let r#type = Type::contract(
            Some(statement.location),
            statement.identifier.name.clone(),
            Some(scope_stack.top()),
        );

        let mut analyzer = StatementAnalyzer::new(scope_stack.top(), HashMap::new());
        for statement in statement.statements.into_iter() {
            if let Some(statement) = analyzer.local_contract(statement)? {
                intermediate.push(statement);
            }
        }
        scope_stack.pop();

        Scope::declare_contract(
            scope_stack.top(),
            statement.identifier,
            ScopeTypeItem::new(Some(location), r#type),
        )?;

        Ok(intermediate)
    }
}
