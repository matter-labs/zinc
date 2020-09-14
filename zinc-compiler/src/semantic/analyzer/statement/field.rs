//!
//! The field statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::field::Statement as FieldStatement;

///
/// The field statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a contract storage field.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        statement: FieldStatement,
        index: usize,
    ) -> Result<(), Error> {
        let r#type = Type::try_from_syntax(statement.r#type, scope.clone())?;

        Scope::define_field(scope, statement.identifier, r#type, index)?;

        Ok(())
    }
}
