//!
//! The field statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::field::Statement as FieldStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a contract field declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: FieldStatement) -> Result<(), Error> {
        let r#type = Type::from_syntax_type(statement.r#type, scope.clone())?;

        Scope::declare_variable(
            scope,
            statement.identifier,
            ScopeVariableItem::new(statement.location, true, r#type),
        )?;

        Ok(())
    }
}
