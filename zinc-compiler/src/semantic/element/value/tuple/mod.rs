//!
//! The semantic analyzer tuple value element.
//!

use std::fmt;

use crate::semantic::Type;

#[derive(Default, Clone, PartialEq)]
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
        Type::new_tuple(self.element_types.clone())
    }

    pub fn push(&mut self, r#type: Type) {
        self.element_types.push(r#type);
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

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.element_types
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
