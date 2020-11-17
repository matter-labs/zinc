//!
//! The `struct` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::StructStatement;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The `struct` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a compile-time only structure type.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: StructStatement) -> Result<Type, Error> {
        let mut fields: Vec<(String, Type)> = Vec::with_capacity(statement.fields.len());
        for field in statement.fields.into_iter() {
            if fields
                .iter()
                .any(|(name, _type)| name == &field.identifier.name)
            {
                return Err(Error::TypeDuplicateField {
                    location: field.location,
                    r#type: statement.identifier.name,
                    field_name: field.identifier.name,
                });
            }

            fields.push((
                field.identifier.name,
                Type::try_from_syntax(field.r#type, scope.clone())?,
            ));
        }

        let r#type = Type::structure(
            Some(statement.location),
            statement.identifier.name,
            fields,
            None,
            scope,
        );

        if !r#type.is_instantiatable(false) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement.location,
                found: r#type.to_string(),
            });
        }

        Ok(r#type)
    }
}
