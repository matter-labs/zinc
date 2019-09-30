//!
//! The interpreter array.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::interpreter::Value;
use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub struct Array {
    pub elements: Vec<Value>,
    pub type_variant: TypeVariant,
}

impl Array {
    pub fn new(type_variant: TypeVariant) -> Self {
        Self {
            elements: Vec::new(),
            type_variant,
        }
    }

    pub fn with_capacity(type_variant: TypeVariant, capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
            type_variant,
        }
    }

    pub fn push(&mut self, value: Value) {
        self.elements.push(value)
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.len() == other.len() && self.type_variant == other.type_variant
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
