//!
//! The interpreter element structure value.
//!

mod error;

pub use self::error::Error;

use std::collections::BTreeMap;
use std::fmt;

use parser::TypeVariant;

use crate::element::Value;

#[derive(Default, Clone, PartialEq)]
pub struct Structure {
    identifier: String,
    fields: BTreeMap<String, TypeVariant>,
    elements: BTreeMap<String, Value>,
}

impl Structure {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        TypeVariant::new_structure(
            self.identifier.clone(),
            self.fields
                .clone()
                .into_iter()
                .collect::<Vec<(String, TypeVariant)>>(),
        )
    }

    pub fn push(&mut self, key: String, value: Value) -> Result<(), Error> {
        if self.fields.contains_key(&key) {
            return Err(Error::FieldAlreadyExists(key));
        }

        self.fields.insert(key.clone(), value.type_variant());
        self.elements.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.elements.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.elements.get_mut(key)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.elements
                .iter()
                .map(|(identifier, type_variant)| format!("{}: {}", identifier, type_variant))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
