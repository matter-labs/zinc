//!
//! The semantic analyzer constant structure element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;

use self::error::Error;

///
/// Structures are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    pub location: Location,
    pub r#type: StructureType,
    pub values: Vec<(String, Constant)>,
}

impl Structure {
    pub fn new(location: Location, r#type: StructureType) -> Self {
        Self {
            location,
            r#type,
            values: vec![],
        }
    }

    pub fn r#type(&self) -> Type {
        Type::Structure(self.r#type.to_owned())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type.unique_id == other.r#type.unique_id
    }

    pub fn push(&mut self, name: String, value: Constant) -> Result<(), Error> {
        match self.r#type.fields.get(self.values.len()) {
            Some((expected_name, expected_type)) => {
                if &name != expected_name {
                    return Err(Error::FieldExpected {
                        type_identifier: self.r#type.identifier.to_owned(),
                        position: self.values.len() + 1,
                        expected: expected_name.to_owned(),
                        found: name,
                    });
                }

                let r#type = value.r#type();
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
                    found: self.values.len() + 1,
                });
            }
        }

        self.values.push((name, value));

        Ok(())
    }

    pub fn slice(self, field_name: String) -> Result<(Constant, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, value)) in self.values.into_iter().enumerate() {
            let element_size = value.r#type().size();

            if name == field_name.as_str() {
                let access = FieldAccess::new(index, offset, element_size, total_size);

                return Ok((value, access));
            }

            offset += element_size;
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
            "constant structure '{}' with fields {{ {} }}",
            self.r#type.identifier,
            self.values
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
