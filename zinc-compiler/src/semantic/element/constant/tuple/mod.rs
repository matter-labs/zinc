//!
//! The semantic analyzer constant tuple element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;

use self::error::Error;

///
/// Tuples are collections of elements of different types.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tuple {
    pub values: Vec<Constant>,
}

impl Tuple {
    pub fn new(values: Vec<Constant>) -> Self {
        Self { values }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn r#type(&self) -> Type {
        Type::tuple(self.values.iter().map(|value| value.r#type()).collect())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.values == other.values
    }

    pub fn push(&mut self, value: Constant) {
        self.values.push(value);
    }

    pub fn slice(mut self, index: usize) -> Result<(Constant, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.values.len() {
            return Err(Error::FieldDoesNotExist {
                type_identifier: self.r#type().to_string(),
                field_index: index,
            });
        }

        let mut tuple_index = 0;
        while tuple_index < index {
            offset += self.values[tuple_index].r#type().size();
            tuple_index += 1;
        }

        let element_size = self.values[tuple_index].r#type().size();

        let access = FieldAccess::new(index, offset, element_size, total_size);

        Ok((self.values.remove(index), access))
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "constant tuple ( {} )",
            self.values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
