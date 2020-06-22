//!
//! The semantic analyzer tuple value element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;

use self::error::Error;

///
/// Tuples are collections of elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    pub location: Option<Location>,
    pub element_types: Vec<Type>,
}

impl Tuple {
    pub fn new(location: Option<Location>) -> Self {
        Self {
            location,
            element_types: vec![],
        }
    }

    pub fn with_capacity(location: Option<Location>, capacity: usize) -> Self {
        Self {
            location,
            element_types: Vec::with_capacity(capacity),
        }
    }

    pub fn new_with_values(location: Option<Location>, element_types: Vec<Type>) -> Self {
        Self {
            location,
            element_types,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::tuple(self.location, self.element_types.to_owned())
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

    pub fn slice(self, index: TupleIndex) -> Result<(Value, StackFieldAccess), Error> {
        let TupleIndex {
            location,
            value: index,
        } = index;

        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.element_types.len() {
            return Err(Error::FieldDoesNotExist {
                location,
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
        let element_size = sliced_type.size();

        let access =
            StackFieldAccess::new(index.to_string(), index, offset, element_size, total_size);

        let result = Value::try_from_type(&sliced_type, self.location)
            .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

        Ok((result, access))
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<runtime> of types {}",
            self.element_types
                .iter()
                .map(|r#type| format!("'{}'", r#type))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
