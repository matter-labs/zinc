//!
//! The interpreter scope.
//!

mod error;

pub use self::error::Error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::interpreter::Place;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<String, Place>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub fn declare_variable(&mut self, place: Place) -> Result<(), Error> {
        if self.is_variable_declared(&place.identifier.name) {
            return Err(Error::RedeclaredVariable(place.identifier.name));
        }
        self.variables.insert(place.identifier.name.clone(), place);
        Ok(())
    }

    pub fn get_variable(&self, identifier: &str) -> Result<Place, Error> {
        if let Some(place) = self.variables.get(identifier) {
            Ok(place.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(identifier),
                None => Err(Error::UndeclaredVariable(identifier.to_owned())),
            }
        }
    }

    pub fn update_variable(&mut self, place: Place) -> Result<(), Error> {
        if let Some(inner) = self.variables.get_mut(&place.identifier.name) {
            if !inner.is_mutable {
                return Err(Error::MutatingImmutableVariable(place.identifier.name));
            }
            inner.value = place.value;
            Ok(())
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_variable(place),
                None => Err(Error::UndeclaredVariable(place.identifier.name)),
            }
        }
    }

    fn is_variable_declared(&self, identifier: &str) -> bool {
        if self.variables.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(identifier),
                None => false,
            }
        }
    }
}
