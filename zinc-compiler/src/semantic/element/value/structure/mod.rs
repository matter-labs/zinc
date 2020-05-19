//!
//! The semantic analyzer structure value element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::syntax::tree::identifier::Identifier;

use self::error::Error;

///
/// Structures are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    pub location: Option<Location>,
    pub r#type: StructureType,
    pub field_index: usize,
}

impl Structure {
    pub fn new(location: Option<Location>, r#type: StructureType) -> Self {
        Self {
            location,
            r#type,
            field_index: 0,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::Structure(self.r#type.to_owned())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }

    pub fn push(
        &mut self,
        identifier: Identifier,
        r#type: Type,
        location: Option<Location>,
    ) -> Result<(), Error> {
        match self.r#type.fields.get(self.field_index) {
            Some((expected_name, expected_type)) => {
                if &identifier.name != expected_name {
                    return Err(Error::FieldExpected {
                        location: identifier.location,
                        type_identifier: self.r#type.identifier.to_owned(),
                        position: self.field_index + 1,
                        expected: expected_name.to_owned(),
                        found: identifier.name,
                    });
                }

                if &r#type != expected_type {
                    return Err(Error::FieldInvalidType {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        type_identifier: self.r#type.identifier.to_owned(),
                        field_name: expected_name.to_owned(),
                        expected: expected_type.to_string(),
                        found: r#type.to_string(),
                    });
                }
            }
            None => {
                return Err(Error::FieldOutOfRange {
                    location: identifier.location,
                    type_identifier: self.r#type.identifier.to_owned(),
                    expected: self.r#type.fields.len(),
                    found: self.field_index + 1,
                });
            }
        }

        self.field_index += 1;

        Ok(())
    }

    pub fn slice(self, identifier: Identifier) -> Result<(Value, StackFieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, r#type)) in self.r#type.fields.iter().enumerate() {
            if name == identifier.name.as_str() {
                let access = StackFieldAccess::new(index, offset, r#type.size(), total_size);

                let result = Value::try_from_type(r#type, self.location)
                    .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

                return Ok((result, access));
            }
            offset += r#type.size();
        }

        Err(Error::FieldDoesNotExist {
            location: identifier.location,
            type_identifier: self.r#type.identifier,
            field_name: identifier.name,
        })
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<runtime> '{}' with fields {}",
            self.r#type.identifier,
            self.r#type
                .fields
                .iter()
                .map(|(name, r#type)| format!("'{}' of type '{}'", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
