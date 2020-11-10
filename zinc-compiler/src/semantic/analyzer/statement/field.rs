//!
//! The field statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::FieldStatement;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

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

        if !r#type.is_instantiatable(true) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement.location,
                found: r#type.to_string(),
            });
        }

        Scope::define_field(
            scope,
            statement.identifier,
            r#type,
            index,
            statement.is_public,
            false,
            false,
        )?;

        Ok(())
    }
}
