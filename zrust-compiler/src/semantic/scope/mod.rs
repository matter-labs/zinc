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
use std::collections::HashSet;
use std::rc::Rc;
use std::str;

use crate::semantic::Place;
use crate::semantic::Value;
use crate::syntax::TypeVariant;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    stack_size: usize,
    items: HashMap<String, Item>,
    constants: HashMap<String, Value>,
    variables: HashMap<String, Variable>,
    types: HashMap<String, TypeVariant>,
    modules: HashSet<String>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size
    }

    pub fn stack_size_recursive(&self) -> usize {
        self.stack_size
            + match self.parent.as_ref() {
                Some(parent) => parent.borrow().stack_size_recursive(),
                None => 0,
            }
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: Value,
        is_mutable: bool,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&name) {
            return Err(Error::RedeclaredItem(name));
        }
        self.variables.insert(
            name.clone(),
            Variable::new(value, self.stack_size_recursive(), is_mutable),
        );
        self.items.insert(name, Item::Variable);
        self.stack_size += 1;
        Ok(())
    }

    pub fn get_variable(&self, place: &Place) -> Result<Variable, Error> {
        if let Some(variable) = self.variables.get(&place.identifier.name) {
            Ok(variable.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(place),
                None => Err(Error::UndeclaredItem(place.identifier.name.to_owned())),
            }
        }
    }

    pub fn update_variable(&mut self, place: &Place, new_value: Value) -> Result<(), Error> {
        let new_address = self.stack_size_recursive();
        if let Some(variable) = self.variables.get_mut(&place.identifier.name) {
            if !variable.is_mutable {
                return Err(Error::MutatingImmutable(place.identifier.name.to_owned()));
            }
            let value = &mut variable.value;
            if !value.has_the_same_type_as(&new_value) {
                return Err(Error::VariableTypeMismatch(
                    new_value.type_variant(),
                    value.type_variant(),
                ));
            }
            *value = new_value;
            variable.address = new_address;
            self.stack_size += 1;
            Ok(())
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_variable(place, new_value),
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
}
