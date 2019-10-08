//!
//! The interpreter element array value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use parser::TypeVariant;

use crate::Value;

#[derive(Clone, PartialEq)]
pub struct Array {
    elements: Vec<Value>,
    type_variant: TypeVariant,
}

impl Default for Array {
    fn default() -> Self {
        Self {
            elements: Default::default(),
            type_variant: TypeVariant::new_unit(),
        }
    }
}

impl Array {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
            type_variant: TypeVariant::new_unit(),
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        TypeVariant::new_array(self.type_variant.clone(), self.elements.len())
    }

    pub fn push(&mut self, value: Value) -> Result<(), Error> {
        if self.elements.is_empty() {
            self.type_variant = value.type_variant();
        } else {
            let type_variant = value.type_variant();
            if type_variant != self.type_variant {
                return Err(Error::PushingInvalidType(
                    type_variant,
                    self.type_variant.clone(),
                ));
            }
        }

        self.elements.push(value);
        Ok(())
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
        self.len() == other.len() && self.type_variant == other.type_variant
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
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
