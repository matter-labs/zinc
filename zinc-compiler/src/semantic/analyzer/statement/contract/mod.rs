//!
//! The `contract` statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only contract declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: ContractStatement) -> Result<Type, Error> {
        let scope = Scope::new_child(scope);

        let r#type = Type::contract(
            Some(statement.location),
            statement.identifier.name.clone(),
            Some(scope.clone()),
        );
        StatementAnalyzer::contract(statement, scope)?;

        Ok(r#type)
    }
}
