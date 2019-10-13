//!
//! The interpreter element place.
//!

mod descriptor;
mod error;

pub use self::descriptor::Descriptor;
pub use self::error::Error;

use std::fmt;

use crate::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub identifier: String,
    pub elements: Vec<Descriptor>,
}

impl Place {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            elements: Vec::new(),
        }
    }

    pub fn index(&mut self, value: Value) -> Result<(), Error> {
        let index = match value {
            Value::Integer(integer) => {
                usize::from_str_radix(integer.to_string().as_str(), crate::BASE_HEXADECIMAL as u32)
                    .expect("Always valid")
            }
            value => return Err(Error::IndexingExpectedIntegerConstant(value)),
        };

        self.elements.push(Descriptor::ArrayIndex(index));
        Ok(())
    }

    pub fn access_tuple(&mut self, value: Value) -> Result<(), Error> {
        let field = match value {
            Value::Integer(integer) => {
                usize::from_str_radix(integer.to_string().as_str(), crate::BASE_HEXADECIMAL as u32)
                    .expect("Always valid")
            }
            value => return Err(Error::TupleAccessExpectedIntegerConstant(value)),
        };

        self.elements.push(Descriptor::TupleField(field));
        Ok(())
    }

    pub fn access_structure(&mut self, place: Place) -> Result<(), Error> {
        let field = place.identifier;

        self.elements.push(Descriptor::StructureField(field));
        Ok(())
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indexes = self
            .elements
            .iter()
            .map(|element| match element {
                Descriptor::ArrayIndex(index) => format!("[{}]", index),
                Descriptor::TupleField(index) => format!(".{}", index),
                Descriptor::StructureField(identifier) => format!(".{}", identifier),
            })
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}{}", self.identifier, indexes)
    }
}
