//!
//! The `struct` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::StructStatement;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;
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
                return Err(Error::Element(ElementError::Type(TypeError::Structure(
                    StructureTypeError::DuplicateField {
                        location: field.location,
                        type_identifier: statement.identifier.name,
                        field_name: field.identifier.name,
                    },
                ))));
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
            Some(scope),
        );

        Ok(r#type)
    }
}
