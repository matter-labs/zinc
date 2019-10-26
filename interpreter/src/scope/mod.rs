//!
//! The interpreter scope.
//!

mod error;
mod item;
mod variable;

pub use self::error::Error;
pub use self::item::Item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::str;

use parser::TypeVariant;

use crate::element::Place;
use crate::element::PlaceDescriptor;
use crate::element::Value;

use self::variable::Variable;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
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
            items: Default::default(),
            constants: Default::default(),
            variables: Default::default(),
            types: Default::default(),
            modules: Default::default(),
        }
    }

    pub fn get_value(&self, place: &Place) -> Result<Value, Error> {
        if let Some(variable) = self.variables.get(&place.identifier) {
            let mut value = &variable.value;
            for index in place.elements.iter() {
                value = match value {
                    Value::Array(array) => {
                        let index = match index {
                            PlaceDescriptor::ArrayIndex(index) => *index,
                            PlaceDescriptor::TupleField(index) => {
                                return Err(Error::ArrayAccessAsTuple(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::StructureField(identifier) => {
                                return Err(Error::ArrayAccessAsStructure(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        array.get(index).ok_or_else(|| {
                            Error::ArrayIndexOutOfRange(index, place.identifier.to_owned())
                        })?
                    }
                    Value::Tuple(tuple) => {
                        let index = match index {
                            PlaceDescriptor::ArrayIndex(index) => {
                                return Err(Error::TupleAccessAsArray(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::TupleField(index) => *index,
                            PlaceDescriptor::StructureField(identifier) => {
                                return Err(Error::TupleAccessAsStructure(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        tuple.get(index).ok_or_else(|| {
                            Error::TupleFieldNotExists(index, place.identifier.to_owned())
                        })?
                    }
                    Value::Structure(structure) => {
                        let identifier = match index {
                            PlaceDescriptor::ArrayIndex(index) => {
                                return Err(Error::StructureAccessAsArray(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::TupleField(index) => {
                                return Err(Error::StructureAccessAsTuple(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::StructureField(identifier) => identifier,
                        };

                        structure.get(identifier).ok_or_else(|| {
                            Error::StructureFieldNotExists(
                                identifier.to_owned(),
                                place.identifier.to_owned(),
                            )
                        })?
                    }
                    _ => {
                        return Err(Error::AddressingPrimitiveTypeVariable(
                            place.identifier.to_owned(),
                        ))
                    }
                };
            }
            Ok(value.to_owned())
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_value(place),
                None => Err(Error::UndeclaredItem(place.identifier.to_owned())),
            }
        }
    }

    pub fn update_value(&mut self, place: &Place, new_value: Value) -> Result<(), Error> {
        if let Some(variable) = self.variables.get_mut(&place.identifier) {
            if !variable.is_mutable {
                return Err(Error::MutatingImmutable(place.identifier.to_owned()));
            }
            let mut value = &mut variable.value;
            for index in place.elements.iter() {
                match value {
                    Value::Array(array) => {
                        let index = match index {
                            PlaceDescriptor::ArrayIndex(index) => *index,
                            PlaceDescriptor::TupleField(index) => {
                                return Err(Error::ArrayAccessAsTuple(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::StructureField(identifier) => {
                                return Err(Error::ArrayAccessAsStructure(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        match array.get_mut(index) {
                            Some(element) => value = element,
                            None => {
                                return Err(Error::ArrayIndexOutOfRange(
                                    index,
                                    place.identifier.to_owned(),
                                ))
                            }
                        }
                    }
                    Value::Tuple(tuple) => {
                        let index = match index {
                            PlaceDescriptor::ArrayIndex(index) => {
                                return Err(Error::TupleAccessAsArray(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::TupleField(index) => *index,
                            PlaceDescriptor::StructureField(identifier) => {
                                return Err(Error::TupleAccessAsStructure(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        match tuple.get_mut(index) {
                            Some(element) => value = element,
                            None => {
                                return Err(Error::TupleFieldNotExists(
                                    index,
                                    place.identifier.to_owned(),
                                ))
                            }
                        }
                    }
                    Value::Structure(structure) => {
                        let identifier = match index {
                            PlaceDescriptor::ArrayIndex(index) => {
                                return Err(Error::StructureAccessAsArray(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::TupleField(index) => {
                                return Err(Error::StructureAccessAsTuple(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceDescriptor::StructureField(identifier) => identifier,
                        };

                        match structure.get_mut(identifier) {
                            Some(element) => value = element,
                            None => {
                                return Err(Error::StructureFieldNotExists(
                                    identifier.to_owned(),
                                    place.identifier.to_owned(),
                                ))
                            }
                        }
                    }
                    _ => {
                        return Err(Error::AddressingPrimitiveTypeVariable(
                            place.identifier.to_owned(),
                        ))
                    }
                }
            }
            if !value.has_the_same_type_as(&new_value) {
                return Err(Error::AssignmentInvalidType(
                    new_value.type_variant(),
                    value.type_variant(),
                ));
            }
            *value = new_value;
            Ok(())
        } else {
            match self.parent {
                Some(ref mut parent) => parent.borrow_mut().update_value(place, new_value),
                None => Err(Error::UndeclaredItem(place.identifier.to_owned())),
            }
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
        self.variables
            .insert(name.clone(), Variable::new(value, is_mutable));
        self.items.insert(name, Item::Variable);
        Ok(())
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
