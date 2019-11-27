//!
//! The semantic analyzer scope.
//!

mod assignment;
mod declaration;
mod error;
mod item;

pub use self::assignment::Assignment;
pub use self::declaration::Declaration;
pub use self::error::Error;
pub use self::item::Item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::semantic::Place;
use crate::syntax::TypeVariant;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,

    declarations: HashMap<String, Declaration>,
    assignments: HashMap<Place, Assignment>,

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
        identifier: String,
        type_variant: TypeVariant,
        is_mutable: bool,
        address: usize,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&identifier) {
            return Err(Error::RedeclaredItem(identifier));
        }
        self.items.insert(identifier.clone(), Item::Variable);
        self.declarations.insert(
            identifier.clone(),
            Declaration::new(type_variant.clone(), is_mutable),
        );
        self.assignments.insert(
            Place::new(identifier),
            Assignment::new(type_variant, address, false),
        );
        Ok(())
    }

    pub fn update_variable(&mut self, place: Place, address: usize) -> Result<(), Error> {
        match self.declarations.get_mut(&place.identifier) {
            Some(declaration) => {
                if !declaration.is_mutable {
                    return Err(Error::MutatingImmutable(place.identifier.to_owned()));
                }
                self.assignments.insert(
                    place,
                    Assignment::new(declaration.type_variant.clone(), address, false),
                );
            }
            None => match self.parent {
                Some(ref parent) => {
                    let declaration = parent.borrow().get_declaration(&place.identifier)?;
                    if !declaration.is_mutable {
                        return Err(Error::MutatingImmutable(place.identifier.to_owned()));
                    }
                    self.assignments.insert(
                        place,
                        Assignment::new(declaration.type_variant, address, true),
                    );
                }
                None => return Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            },
        }

        Ok(())
    }

    pub fn get_declaration(&self, identifier: &str) -> Result<Declaration, Error> {
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
        if let Some(assignment) = self.assignments.get(place) {
            Ok(assignment.address)
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable_address(place),
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn declare_type(
        &mut self,
        identifier: String,
        type_variant: TypeVariant,
    ) -> Result<(), Error> {
        if let Ok(_item) = self.get_item_type(&identifier) {
            return Err(Error::RedeclaredItem(identifier));
        }
        self.types.insert(identifier.clone(), type_variant);
        self.items.insert(identifier, Item::Type);
        Ok(())
    }

    pub fn resolve_type(&self, identifier: &str) -> Result<TypeVariant, Error> {
        match self.types.get(identifier) {
            Some(TypeVariant::Alias { identifier }) => self.resolve_type(identifier),
            Some(type_variant) => Ok(type_variant.to_owned()),
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

    pub fn get_ordered_outer_assignments(&self) -> Vec<(Place, Assignment)> {
        let mut assignments = self
            .assignments
            .iter()
            .filter(|(_place, assignment)| assignment.is_outer)
            .map(|(place, assignment)| (place.to_owned(), assignment.to_owned()))
            .collect::<Vec<(Place, Assignment)>>();
        assignments.sort_by(|(_, a), (_, b)| a.address.cmp(&b.address));
        assignments
    }
}
