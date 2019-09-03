//!
//! The interpreter scope.
//!

use std::collections::HashMap;

use crate::interpreter::Place;
use crate::syntax::Identifier;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Box<Self>>,
    variables: HashMap<Vec<u8>, Place>,
}

impl Scope {
    pub fn is_variable_declared(&self, identifier: &Identifier) -> bool {
        if self.variables.contains_key(&identifier.name) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.is_variable_declared(identifier),
                None => false,
            }
        }
    }

    pub fn get_variable(&self, identifier: &Identifier) -> Option<&Place> {
        self.variables.get(&identifier.name)
    }

    pub fn declare_variable(&mut self, place: Place) {
        self.variables.insert(place.identifier.clone().name, place);
    }

    pub fn update_variable(&mut self, place: Place) -> Option<()> {
        if let Some(inner) = self.variables.get_mut(&place.identifier.name) {
            inner.value = place.value;
            Some(())
        } else {
            None
        }
    }
}
