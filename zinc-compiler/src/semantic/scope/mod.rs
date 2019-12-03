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
use crate::semantic::Type;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
    declarations: HashMap<String, Variable>,
    types: HashMap<String, Type>,
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
        identifier: String,
        r#type: Type,
        is_mutable: bool,
        address: usize,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&identifier) {
            return Err(Error::RedeclaredItem(identifier));
        }
        self.items.insert(identifier.clone(), Item::Variable);
        self.declarations.insert(
            identifier.clone(),
            Variable::new(r#type, is_mutable, address),
        );
        Ok(())
    }

    pub fn check_assignment(&mut self, place: &Place) -> Result<(), Error> {
        match self.declarations.get_mut(&place.identifier) {
            Some(declaration) => {
                if !declaration.is_mutable {
                    return Err(Error::MutatingImmutable(place.identifier.to_owned()));
                }
            }
            None => match self.parent {
                Some(ref parent) => parent.borrow_mut().check_assignment(place)?,
                None => return Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            },
        }

        Ok(())
    }

    pub fn get_declaration(&self, identifier: &str) -> Result<Variable, Error> {
        if let Some(declaration) = self.declarations.get(identifier) {
            Ok(declaration.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_declaration(identifier),
                None => Err(Error::UndeclaredVariable(identifier.to_owned())),
            }
        }
    }

    pub fn get_variable_address(&self, place: &Place) -> Result<usize, Error> {
        if let Some(declaration) = self.declarations.get(&place.identifier) {
            Ok(declaration.address)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_address(place),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn declare_type(&mut self, identifier: String, r#type: Type) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&identifier) {
            return Err(Error::RedeclaredItem(identifier));
        }
        self.types.insert(identifier.clone(), r#type);
        self.items.insert(identifier, Item::Type);
        Ok(())
    }

    pub fn resolve_type(&self, identifier: &str) -> Result<Type, Error> {
        match self.types.get(identifier) {
            //            Some(TypeVariant::Alias { identifier }) => self.resolve_type(identifier),
            Some(r#type) => Ok(r#type.to_owned()),
            None => match self.parent {
                Some(ref parent) => parent.borrow().resolve_type(identifier),
                None => Err(Error::UndeclaredType(identifier.to_owned())),
            },
        }
    }

    pub fn get_item_type(&self, identifier: &str) -> Result<Item, Error> {
        if let Some(item) = self.items.get(identifier).copied() {
            Ok(item)
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get_item_type(identifier)
        } else {
            Err(Error::UndeclaredItem(identifier.to_owned()))
        }
    }
}
