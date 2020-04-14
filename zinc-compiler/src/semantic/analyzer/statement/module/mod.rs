//!
//! The `mod` statement semantic analyzer.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::module::error::Error as ModStatementError;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::module::Statement as ModStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile time only module declaration statement.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: ModStatement,
        dependencies: &mut HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<(), Error> {
        let identifier_location = statement.identifier.location;
        let module = match dependencies.remove(statement.identifier.name.as_str()) {
            Some(module) => module,
            None => {
                return Err(Error::Statement(StatementError::Mod(
                    ModStatementError::NotFound {
                        location: identifier_location,
                        name: statement.identifier.name,
                    },
                )));
            }
        };
        Scope::declare_module(scope, statement.identifier, module)?;

        Ok(())
    }
}
