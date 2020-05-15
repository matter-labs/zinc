//!
//! The field statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::memory_type::MemoryType;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::field::Statement as FieldStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a contract field declaration statement.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: FieldStatement,
        index: usize,
    ) -> Result<(), Error> {
        let r#type = Type::from_syntax_type(statement.r#type, scope.clone())?;

        Scope::define_variable(
            scope,
            statement.identifier,
            false,
            r#type,
            MemoryType::ContractStorage { index },
        )?;

        Ok(())
    }
}
