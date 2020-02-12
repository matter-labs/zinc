//!
//! The semantic analyzer structure value element.
//!

pub mod error;

use std::fmt;

use crate::semantic::element::access::AccessData;
use crate::semantic::element::r#type::Type;

use self::error::Error;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Structure {
    identifier: String,
    unique_id: usize,
    fields: Vec<(String, Type)>,
}

impl Structure {
    pub fn new(identifier: String, unique_id: usize, fields: Vec<(String, Type)>) -> Self {
        Self {
            identifier,
            unique_id,
            fields,
        }
    }

    pub fn slice(&self, field_name: &str) -> Result<AccessData, Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (name, r#type) in self.fields.iter() {
            if name == field_name {
                return Ok(AccessData::new(
                    offset,
                    r#type.size(),
                    total_size,
                    r#type.to_owned(),
                ));
            }
            offset += r#type.size();
        }

        Err(Error::FieldDoesNotExist(
            field_name.to_owned(),
            self.identifier.to_string(),
        ))
    }

    pub fn r#type(&self) -> Type {
        Type::structure(
            self.identifier.to_owned(),
            self.unique_id,
            self.fields.to_owned(),
            None,
        )
    }

    pub fn push(&mut self, key: String, r#type: Type) -> Result<(), Error> {
        if self.fields.iter().any(|field| field.0 == key) {
            return Err(Error::FieldAlreadyExists(key, self.identifier.to_owned()));
        }

        self.fields.push((key, r#type));
        Ok(())
    }

    pub fn contains_key(&mut self, key: &str) -> bool {
        self.fields.iter().any(|field| field.0 == key)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type())
    }
}
