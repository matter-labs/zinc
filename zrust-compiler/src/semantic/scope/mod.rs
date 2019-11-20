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

    variables: HashMap<Place, Variable>,
    assignments: HashMap<Place, usize>,

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
        place: Place,
        value: Value,
        is_mutable: bool,
        address: usize,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&place.identifier) {
            return Err(Error::RedeclaredItem(place.identifier));
        }
        self.items.insert(place.identifier.clone(), Item::Variable);
        self.variables
            .insert(place.clone(), Variable::new(value, is_mutable));
        self.assignments.insert(place, address);
        Ok(())
    }

    pub fn update_variable(&mut self, place: Place, address: usize) -> Result<(), Error> {
        if let Some(variable) = self.variables.get_mut(&place) {
            if !variable.is_mutable {
                return Err(Error::MutatingImmutable(place.identifier));
            }
        } else {
            match self.parent {
                Some(ref mut parent) => parent
                    .borrow_mut()
                    .update_variable(place.clone(), address)?,
                None => return Err(Error::UndeclaredVariable(place.identifier)),
            }
        }

        self.assignments.insert(place, address);
        Ok(())
    }

    pub fn get_variable_value(&self, place: &Place) -> Result<Value, Error> {
        if let Some(variable) = self.variables.get(place) {
            Ok(variable.value.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_value(place),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn get_variable_address(&self, place: &Place) -> Result<usize, Error> {
        if let Some(address) = self.assignments.get(place).copied() {
            Ok(address)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_address(place),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
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
                None => Err(Error::UndeclaredType(name.to_owned())),
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

    pub fn get_assignments(&self) -> HashMap<Place, usize> {
        self.assignments.clone()
    }

    pub fn add_assignments(&mut self, assignments: HashMap<Place, usize>) {
        self.assignments.extend(assignments)
    }
}
