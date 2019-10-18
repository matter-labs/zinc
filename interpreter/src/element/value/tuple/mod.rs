//!
//! The interpreter element tuple value.
//!

use std::fmt;

use parser::TypeVariant;

use crate::element::Value;

#[derive(Default, Clone, PartialEq)]
pub struct Tuple {
    elements: Vec<Value>,
    type_variants: Vec<TypeVariant>,
}

impl Tuple {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
            type_variants: Vec::with_capacity(capacity),
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        TypeVariant::new_tuple(self.type_variants.clone())
    }

    pub fn push(&mut self, value: Value) {
        self.type_variants.push(value.type_variant());
        self.elements.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.elements.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.type_variants == other.type_variants
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
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
