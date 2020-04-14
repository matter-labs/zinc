//!
//! The `contract` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::contract::error::Error as ContractTypeError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::r#type::INDEX as TYPE_INDEX;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::contract::Statement as StructStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only contract declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: StructStatement) -> Result<(), Error> {
        let mut fields: Vec<(String, Type)> = Vec::with_capacity(statement.fields.len());
        for field in statement.fields.into_iter() {
            if fields
                .iter()
                .any(|(name, _type)| name == &field.identifier.name)
            {
                return Err(Error::Element(
                    field.location,
                    ElementError::Type(TypeError::Contract(ContractTypeError::DuplicateField {
                        type_identifier: statement.identifier.name,
                        field_name: field.identifier.name,
                    })),
                ));
            }

            fields.push((
                field.identifier.name,
                Type::from_type_variant(&field.r#type.variant, scope.clone())?,
            ));
        }

        let unique_id = TYPE_INDEX.read().expect(crate::panic::MUTEX_SYNC).len();
        let r#type = Type::contract(
            statement.identifier.name.clone(),
            unique_id,
            fields,
            Some(Rc::new(RefCell::new(Scope::new(Some(scope.clone()))))),
        );

        TYPE_INDEX
            .write()
            .expect(crate::panic::MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());
        Scope::declare_type(scope, statement.identifier, r#type)?;

        Ok(())
    }
}
