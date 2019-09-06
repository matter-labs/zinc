//!
//! The interpreter scope.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use failure::Fail;
use serde_derive::Serialize;

use crate::interpreter::Place;
use crate::syntax::Identifier;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<String, Place>,
}

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared variable: '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "mutating an immutable variable: '{}'", _0)]
    MutatingImmutableVariable(String),
}

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Warning {
    #[fail(display = "redeclared variable: '{}'", _0)]
    RedeclaredVariable(String),
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub fn get_variable(&self, identifier: &Identifier) -> Result<Place, Error> {
        if let Some(place) = self.variables.get(identifier.name()).cloned() {
            Ok(place)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(identifier),
                None => Err(Error::UndeclaredVariable(identifier.name().to_owned())),
            }
        }
    }

    pub fn declare_variable(&mut self, place: Place) -> Option<Warning> {
        let warning = if self.is_variable_declared(&place.identifier) {
            Some(Warning::RedeclaredVariable(
                place.identifier.name().to_owned(),
            ))
        } else {
            None
        };
        self.variables
            .insert(place.identifier.name().to_owned(), place);
        warning
    }

    pub fn update_variable(&mut self, place: Place) -> Result<(), Error> {
        if let Some(inner) = self.variables.get_mut(place.identifier.name()) {
            if !inner.is_mutable {
                return Err(Error::MutatingImmutableVariable(
                    place.identifier.name().to_owned(),
                ));
            }
            inner.value = place.value;
            Ok(())
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_variable(place),
                None => Err(Error::UndeclaredVariable(
                    place.identifier.name().to_owned(),
                )),
            }
        }
    }

    fn is_variable_declared(&self, identifier: &Identifier) -> bool {
        if self.variables.contains_key(identifier.name()) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(identifier),
                None => false,
            }
        }
    }
}
