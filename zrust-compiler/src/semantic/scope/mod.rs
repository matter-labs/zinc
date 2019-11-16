//!
//! The semantic analyzer scope.
//!

mod error;
mod item;
mod variable;

pub use self::error::Error;
pub use self::item::Item;
pub use self::variable::Variable;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::semantic::Place;
use crate::semantic::Value;
use crate::syntax::TypeVariant;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,

    variables: HashMap<String, Variable>,
    assignments: HashMap<String, usize>,

    types: HashMap<String, TypeVariant>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: Value,
        is_mutable: bool,
        address: usize,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&name) {
            return Err(Error::RedeclaredItem(name));
        }
        self.variables
            .insert(name.clone(), Variable::new(value, is_mutable));
        self.assignments.insert(name.clone(), address);
        self.items.insert(name, Item::Variable);
        Ok(())
    }

    pub fn update_variable(&mut self, identifier: &str, address: usize) -> Result<(), Error> {
        if let Some(variable) = self.variables.get_mut(identifier) {
            if !variable.is_mutable {
                return Err(Error::MutatingImmutable(identifier.to_owned()));
            }
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_variable(identifier, address)?,
                None => return Err(Error::UndeclaredItem(identifier.to_owned())),
            }
        }

        self.assignments.insert(identifier.to_owned(), address);
        Ok(())
    }

    pub fn get_variable_value(&self, place: &Place) -> Result<Value, Error> {
        if let Some(variable) = self.variables.get(&place.identifier.name) {
            Ok(variable.value.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_value(place),
                None => Err(Error::UndeclaredItem(place.identifier.name.to_owned())),
            }
        }
    }

    pub fn get_variable_address(&self, place: &Place) -> Result<usize, Error> {
        if let Some(address) = self.assignments.get(&place.identifier.name).copied() {
            Ok(address)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_address(place),
                None => Err(Error::UndeclaredItem(place.identifier.name.to_owned())),
            }
        }
    }

    pub fn declare_type(&mut self, name: String, type_variant: TypeVariant) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&name) {
            return Err(Error::RedeclaredItem(name));
        }
        self.types.insert(name.clone(), type_variant);
        self.items.insert(name, Item::Type);
        Ok(())
    }

    pub fn resolve_type(&self, name: &str) -> Result<TypeVariant, Error> {
        match self.types.get(name) {
            Some(TypeVariant::Alias { identifier }) => self.resolve_type(identifier),
            Some(type_variant) => Ok(type_variant.to_owned()),
            None => match self.parent {
                Some(ref parent) => parent.borrow().resolve_type(name),
                None => Err(Error::UndeclaredItem(name.to_owned())),
            },
        }
    }

    pub fn get_item_type(&self, name: &str) -> Result<Item, Error> {
        if let Some(item) = self.items.get(name).copied() {
            Ok(item)
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get_item_type(name)
        } else {
            Err(Error::UndeclaredItem(name.to_owned()))
        }
    }

    pub fn get_assignments(&self) -> HashMap<String, usize> {
        self.assignments.clone()
    }

    pub fn add_assignments(&mut self, assignments: HashMap<String, usize>) {
        self.assignments.extend(assignments)
    }

    pub fn move_assignments(&mut self) {
        for (identifier, address) in self.assignments.drain() {
            if !self.variables.contains_key(&identifier) {
                self.parent
                    .as_mut()
                    .expect("Always contains a value")
                    .borrow_mut()
                    .assignments
                    .insert(identifier, address);
            }
        }
    }
}
