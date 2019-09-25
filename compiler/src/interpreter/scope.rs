//!
//! The interpreter scope.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use failure::Fail;

use bellman::ConstraintSystem;
use pairing::bn256::Bn256;

use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::interpreter::ValueError;
use crate::syntax::Identifier;
use crate::syntax::Input;
use crate::syntax::Witness;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<String, Place>,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "value: '{}'", _0)]
    Value(ValueError),
    #[fail(display = "undeclared variable: '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "redeclared variable: '{}'", _0)]
    RedeclaredVariable(String),
    #[fail(display = "mutating an immutable variable: '{}'", _0)]
    MutatingImmutableVariable(String),
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub fn declare_input<S: ConstraintSystem<Bn256>>(
        &mut self,
        input: Input,
        system: &mut S,
    ) -> Result<(), Error> {
        if self.is_variable_declared(&input.identifier) {
            return Err(Error::RedeclaredVariable(input.identifier.name));
        }
        let place = Place::new(
            input.identifier.to_owned(),
            Value::new_from_type(input.r#type.variant, system).map_err(Error::Value)?,
            false,
        );
        self.variables.insert(input.identifier.name, place);
        Ok(())
    }

    pub fn declare_witness<S: ConstraintSystem<Bn256>>(
        &mut self,
        witness: Witness,
        system: &mut S,
    ) -> Result<(), Error> {
        if self.is_variable_declared(&witness.identifier) {
            return Err(Error::RedeclaredVariable(witness.identifier.name));
        }
        let place = Place::new(
            witness.identifier.to_owned(),
            Value::new_from_type(witness.r#type.variant, system).map_err(Error::Value)?,
            false,
        );
        self.variables.insert(witness.identifier.name, place);
        Ok(())
    }

    pub fn declare_variable(&mut self, place: Place) -> Result<(), Error> {
        if self.is_variable_declared(&place.identifier) {
            return Err(Error::RedeclaredVariable(place.identifier.name));
        }
        self.variables
            .insert(place.identifier.name.to_owned(), place);
        Ok(())
    }

    pub fn get_variable(&self, identifier: &Identifier) -> Result<Place, Error> {
        if let Some(place) = self.variables.get(&identifier.name).cloned() {
            Ok(place)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(identifier),
                None => Err(Error::UndeclaredVariable(identifier.name.to_owned())),
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

    fn is_variable_declared(&self, identifier: &Identifier) -> bool {
        if self.variables.contains_key(&identifier.name) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(identifier),
                None => false,
            }
        }
    }
}
