//!
//! The semantic analyzer tuple value element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;

use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;
use crate::semantic::error::Error;

///
/// Tuples are collections of elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    /// The location, where the tuple appears in the code.
    pub location: Option<Location>,
    /// The tuple element types.
    pub element_types: Vec<Type>,
}

impl Tuple {
    ///
    /// A shortcut constructor, which is called before pushing the element values.
    ///
    pub fn new(location: Option<Location>) -> Self {
        Self {
            location,
            element_types: vec![],
        }
    }

    ///
    /// A shortcut constructor, which is called before pushing the element values.
    ///
    pub fn with_capacity(location: Option<Location>, capacity: usize) -> Self {
        Self {
            location,
            element_types: Vec::with_capacity(capacity),
        }
    }

    ///
    /// A shortcut constructor, which is called when the type and values are already known.
    ///
    pub fn new_with_values(location: Option<Location>, element_types: Vec<Type>) -> Self {
        Self {
            location,
            element_types,
        }
    }

    ///
    /// The tuple fields count.
    ///
    pub fn len(&self) -> usize {
        self.element_types.len()
    }

    ///
    /// If the tuple fields count is exactly zero.
    ///
    pub fn is_empty(&self) -> bool {
        self.element_types.is_empty()
    }

    ///
    /// Pushes a typed element into the tuple fields array.
    ///
    pub fn push(&mut self, r#type: Type) {
        self.element_types.push(r#type);
    }

    ///
    /// Slices the tuple, returning the specified field.
    ///
    pub fn slice(self, index: TupleIndex) -> Result<(Value, StackFieldAccess), Error> {
        let TupleIndex {
            location,
            value: index,
        } = index;

        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.element_types.len() {
            return Err(Error::TupleFieldOutOfRange {
                location,
                r#type: self.r#type().to_string(),
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

        let result = Value::try_from_type(&sliced_type, false, self.location)
            .expect(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

        Ok((result, access))
    }
}

impl ITyped for Tuple {
    fn r#type(&self) -> Type {
        Type::tuple(self.location, self.element_types.to_owned())
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.element_types == other.element_types
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
