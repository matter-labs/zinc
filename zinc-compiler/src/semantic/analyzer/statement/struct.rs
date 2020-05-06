//!
//! The `struct` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#struct::Statement as StructStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only structure declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: StructStatement) -> Result<(), Error> {
        let location = statement.location;

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
                Type::from_syntax_type(field.r#type, scope.clone())?,
            ));
        }

        let namespace_scope = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));
        let r#type = Type::structure(
            Some(statement.location),
            statement.identifier.name.clone(),
            fields,
            Some(namespace_scope),
        );

        Scope::declare_type(
            scope,
            statement.identifier,
            ScopeTypeItem::new(Some(location), r#type),
        )?;

        Ok(())
    }
}
