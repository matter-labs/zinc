//!
//! The semantic analyzer tuple value element.
//!

mod tests;

pub mod error;

use std::convert::TryFrom;
use std::fmt;

use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;

use self::error::Error;

///
/// Tuples are collections of elements of different types.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tuple {
    element_types: Vec<Type>,
}

impl Tuple {
    pub fn new(element_types: Vec<Type>) -> Self {
        Self { element_types }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            element_types: Vec::with_capacity(capacity),
        }
    }

    pub fn r#type(&self) -> Type {
        Type::tuple(self.element_types.to_owned())
    }

    pub fn len(&self) -> usize {
        self.element_types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element_types.is_empty()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.element_types == other.element_types
    }

    pub fn push(&mut self, r#type: Type) {
        self.element_types.push(r#type);
    }

    pub fn slice(self, index: usize) -> Result<(Value, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.element_types.len() {
            return Err(Error::FieldDoesNotExist {
                type_identifier: self.r#type().to_string(),
                field_index: index,
            });
        }

        let mut tuple_index = 0;
        while tuple_index < index {
            offset += self.element_types[tuple_index].size();
            tuple_index += 1;
        }

        let sliced_type = self.element_types[tuple_index].clone();

        let access = FieldAccess::new(index, offset, sliced_type.size(), total_size);

        Ok((
            Value::try_from(&sliced_type).expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
            access,
        ))
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<tuple> of types {}",
            self.element_types
                .iter()
                .map(|r#type| format!("'{}'", r#type))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
