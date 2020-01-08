//!
//! The semantic analyzer place element.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::lexical::Location;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Type;
use crate::semantic::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub location: Location,
    pub r#type: Type,
    pub address: usize,
    pub total_size: usize,
    pub is_mutable: bool,
    pub is_global: bool,
}

impl Place {
    pub fn new(
        location: Location,
        r#type: Type,
        address: usize,
        is_mutable: bool,
        is_global: bool,
    ) -> Self {
        let total_size = r#type.size();
        Self {
            location,
            r#type,
            address,
            total_size,
            is_mutable,
            is_global,
        }
    }

    pub fn index_array(&mut self, index_value: &Element) -> Result<usize, Error> {
        match index_value {
            Element::Value(Value::Integer(..)) => {}
            Element::Constant(Constant::Integer(..)) => {}
            value => {
                return Err(Error::OperatorIndexSecondOperandExpectedInteger(
                    value.to_string(),
                ))
            }
        }

        match self.r#type {
            Type::Array { ref r#type, .. } => {
                self.r#type = *r#type.to_owned();
                Ok(self.r#type.size())
            }
            ref r#type => Err(Error::OperatorIndexFirstOperandExpectedArray(
                r#type.to_string(),
            )),
        }
    }

    pub fn field_tuple(&mut self, field_index: usize) -> Result<usize, Error> {
        let mut offset = 0;
        match self.r#type {
            Type::Tuple { ref types } => {
                if field_index >= types.len() {
                    return Err(Error::FieldDoesNotExistInStructure(
                        field_index.to_string(),
                        self.r#type.to_string(),
                    ));
                }
                let mut tuple_index = 0;
                while tuple_index < field_index {
                    offset += types[tuple_index].size();
                    tuple_index += 1;
                }
                self.r#type = types[tuple_index].to_owned();
                Ok(offset)
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTupleOrStructure(
                r#type.to_string(),
            )),
        }
    }

    pub fn field_structure(&mut self, field_name: &str) -> Result<usize, Error> {
        let mut offset = 0;
        match self.r#type {
            Type::Structure {
                ref identifier,
                ref fields,
                ..
            } => {
                for structure_field in fields.iter() {
                    if structure_field.0 == field_name {
                        self.r#type = structure_field.1.to_owned();
                        return Ok(offset);
                    }
                    offset += structure_field.1.size();
                }
                Err(Error::FieldDoesNotExistInStructure(
                    field_name.to_owned(),
                    identifier.to_string(),
                ))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTupleOrStructure(
                r#type.to_string(),
            )),
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}
