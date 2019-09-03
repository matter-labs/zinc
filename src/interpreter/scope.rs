//!
//! The interpreter scope.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::Place;
use crate::syntax::Identifier;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<Vec<u8>, Place>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub fn is_variable_declared(&self, identifier: &Identifier) -> bool {
        if self.variables.contains_key(&identifier.name) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(identifier),
                None => false,
            }
        }
    }

    pub fn get_variable(&self, identifier: &Identifier) -> Option<Place> {
        if let Some(place) = self.variables.get(&identifier.name).cloned() {
            Some(place)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(identifier),
                None => None,
            }
        }
    }

    pub fn declare_variable(&mut self, place: Place) {
        self.variables.insert(place.identifier.clone().name, place);
    }

    pub fn update_variable(&mut self, place: Place) -> bool {
        if let Some(inner) = self.variables.get_mut(&place.identifier.name) {
            inner.value = place.value;
            true
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_variable(place),
                None => false,
            }
        }
    }
}
