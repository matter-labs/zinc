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
use crate::interpreter::Value;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<String, (bool, Value)>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub fn get_value(&self, place: &Place) -> Result<Value, Error> {
        if let Some((_is_mutable, value)) = self.variables.get(&place.identifier) {
            let mut value = value;
            for index in place.path.iter() {
                value = match value {
                    Value::Array(array) => array.elements.get(*index).ok_or_else(|| {
                        Error::ArrayIndexOutOfRange(*index, place.identifier.to_owned())
                    })?,
                    _ => return Err(Error::IndexingNotArray(place.identifier.to_owned())),
                };
            }
            Ok(value.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_value(place),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn update_value(&mut self, place: &Place, new_value: Value) -> Result<(), Error> {
        if let Some((is_mutable, value)) = self.variables.get_mut(&place.identifier) {
            if !*is_mutable {
                return Err(Error::MutatingImmutableVariable(
                    place.identifier.to_owned(),
                ));
            }
            let mut value = value;
            for index in place.path.iter() {
                match value {
                    Value::Array(array) => match array.elements.get_mut(*index) {
                        Some(element) => value = element,
                        None => {
                            return Err(Error::ArrayIndexOutOfRange(
                                *index,
                                place.identifier.to_owned(),
                            ))
                        }
                    },
                    _ => return Err(Error::IndexingNotArray(place.identifier.to_owned())),
                }
            }
            *value = new_value;
            Ok(())
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_value(place, new_value),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: Value,
        is_mutable: bool,
    ) -> Result<(), Error> {
        if self.is_variable_declared(&name) {
            return Err(Error::RedeclaredVariable(name));
        }
        self.variables.insert(name, (is_mutable, value));
        Ok(())
    }

    fn is_variable_declared(&self, name: &str) -> bool {
        if self.variables.contains_key(name) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(name),
                None => false,
            }
        }
    }
}
