//!
//! The semantic analyzer constant tuple element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;

use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::error::Error;

///
/// Tuples are collections of elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    /// The location, where the tuple appears in the code.
    pub location: Location,
    /// The tuple element values.
    pub values: Vec<Constant>,
}

impl Tuple {
    ///
    /// A shortcut constructor, which is called before pushing the element values.
    ///
    pub fn new(location: Location) -> Self {
        Self {
            location,
            values: vec![],
        }
    }

    ///
    /// A shortcut constructor, which is called before pushing the element values.
    ///
    pub fn with_capacity(location: Location, capacity: usize) -> Self {
        Self {
            location,
            values: Vec::with_capacity(capacity),
        }
    }

    ///
    /// A shortcut constructor, which is called when the type and values are already known.
    ///
    pub fn new_with_values(location: Location, values: Vec<Constant>) -> Self {
        Self { location, values }
    }

    ///
    /// The tuple fields count.
    ///
    pub fn len(&self) -> usize {
        self.values.len()
    }

    ///
    /// If the tuple fields count is exactly zero.
    ///
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    ///
    /// Pushes a typed element into the tuple fields array.
    ///
    pub fn push(&mut self, value: Constant) {
        self.values.push(value);
    }

    ///
    /// Slices the tuple, returning the specified field.
    ///
    pub fn slice(mut self, index: TupleIndex) -> Result<(Constant, StackFieldAccess), Error> {
        let TupleIndex {
            location,
            value: index,
        } = index;

        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.values.len() {
            return Err(Error::TupleFieldOutOfRange {
                location,
                r#type: self.r#type().to_string(),
                field_index: index,
            });
        }

        let mut tuple_index = 0;
        while tuple_index < index {
            offset += self.values[tuple_index].r#type().size();
            tuple_index += 1;
        }

        let element_size = self.values[tuple_index].r#type().size();

        let access =
            StackFieldAccess::new(index.to_string(), index, offset, element_size, total_size);

        Ok((self.values.remove(index), access))
    }
}

impl ITyped for Tuple {
    fn r#type(&self) -> Type {
        Type::tuple(
            Some(self.location),
            self.values.iter().map(|value| value.r#type()).collect(),
        )
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( {} )",
            self.values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
