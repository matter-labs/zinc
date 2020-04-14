//!
//! The semantic analyzer structure value element.
//!

mod tests;

pub mod error;

use std::convert::TryFrom;
use std::fmt;

use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;

use self::error::Error;

///
/// Structures are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    r#type: StructureType,
    field_index: usize,
}

impl Structure {
    pub fn new(r#type: StructureType) -> Self {
        Self {
            r#type,
            field_index: 0,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::Structure(self.r#type.to_owned())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type.unique_id == other.r#type.unique_id
    }

    pub fn push(&mut self, name: String, r#type: Type) -> Result<(), Error> {
        match self.r#type.fields.get(self.field_index) {
            Some((expected_name, expected_type)) => {
                if &name != expected_name {
                    return Err(Error::FieldExpected {
                        type_identifier: self.r#type.identifier.to_owned(),
                        position: self.field_index + 1,
                        expected: expected_name.to_owned(),
                        found: name,
                    });
                }

                if &r#type != expected_type {
                    return Err(Error::FieldInvalidType {
                        type_identifier: self.r#type.identifier.to_owned(),
                        field_name: expected_name.to_owned(),
                        expected: expected_type.to_string(),
                        found: r#type.to_string(),
                    });
                }
            }
            None => {
                return Err(Error::FieldOutOfRange {
                    type_identifier: self.r#type.identifier.to_owned(),
                    expected: self.r#type.fields.len(),
                    found: self.field_index + 1,
                });
            }
        }

        self.field_index += 1;

        Ok(())
    }

    pub fn slice(self, field_name: String) -> Result<(Value, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, r#type)) in self.r#type.fields.iter().enumerate() {
            if name == field_name.as_str() {
                let access = FieldAccess::new(index, offset, r#type.size(), total_size);

                return Ok((
                    Value::try_from(r#type).expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
                    access,
                ));
            }
            offset += r#type.size();
        }

        Err(Error::FieldDoesNotExist {
            type_identifier: self.r#type.identifier,
            field_name,
        })
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<structure> '{}' with fields {}",
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
