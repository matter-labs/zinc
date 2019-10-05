//!
//! The interpreter scope.
//!

mod error;
mod variable;

pub use self::error::Error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use self::variable::Variable;
use crate::interpreter::Place;
use crate::interpreter::PlaceElement;
use crate::interpreter::Value;
use crate::syntax::TypeVariant;

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    variables: HashMap<String, Variable>,
    types: HashMap<String, TypeVariant>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
            types: Default::default(),
        }
    }

    pub fn get_value(&self, place: &Place) -> Result<Value, Error> {
        if let Some(variable) = self.variables.get(&place.identifier) {
            let mut value = &variable.value;
            for index in place.elements.iter() {
                value = match value {
                    Value::Array(array) => {
                        let index = match index {
                            PlaceElement::ArrayIndex(index) => *index,
                            PlaceElement::TupleField(index) => {
                                return Err(Error::ArrayAccessingTupleField(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::StructureField(identifier) => {
                                return Err(Error::ArrayAccessingStructureField(
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
                            PlaceElement::ArrayIndex(index) => {
                                return Err(Error::TupleIndexing(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::TupleField(index) => *index,
                            PlaceElement::StructureField(identifier) => {
                                return Err(Error::TupleAccessingStructureField(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        tuple.get(index).ok_or_else(|| {
                            Error::TupleFieldOutOfRange(index, place.identifier.to_owned())
                        })?
                    }
                    Value::Structure(structure) => {
                        let identifier = match index {
                            PlaceElement::ArrayIndex(index) => {
                                return Err(Error::StructureIndexing(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::TupleField(index) => {
                                return Err(Error::StructureAccessingWithTupleField(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::StructureField(identifier) => identifier,
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
                None => Err(Error::UndeclaredVariable(place.identifier.to_owned())),
            }
        }
    }

    pub fn update_value(&mut self, place: &Place, new_value: Value) -> Result<(), Error> {
        if let Some(variable) = self.variables.get_mut(&place.identifier) {
            if !variable.is_mutable {
                return Err(Error::MutatingImmutableVariable(
                    place.identifier.to_owned(),
                ));
            }
            let mut value = &mut variable.value;
            for index in place.elements.iter() {
                match value {
                    Value::Array(array) => {
                        let index = match index {
                            PlaceElement::ArrayIndex(index) => *index,
                            PlaceElement::TupleField(index) => {
                                return Err(Error::ArrayAccessingTupleField(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::StructureField(identifier) => {
                                return Err(Error::ArrayAccessingStructureField(
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
                            PlaceElement::ArrayIndex(index) => {
                                return Err(Error::TupleIndexing(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::TupleField(index) => *index,
                            PlaceElement::StructureField(identifier) => {
                                return Err(Error::TupleAccessingStructureField(
                                    place.identifier.to_owned(),
                                    identifier.to_owned(),
                                ));
                            }
                        };

                        match tuple.get_mut(index) {
                            Some(element) => value = element,
                            None => {
                                return Err(Error::TupleFieldOutOfRange(
                                    index,
                                    place.identifier.to_owned(),
                                ))
                            }
                        }
                    }
                    Value::Structure(structure) => {
                        let identifier = match index {
                            PlaceElement::ArrayIndex(index) => {
                                return Err(Error::StructureIndexing(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::TupleField(index) => {
                                return Err(Error::StructureAccessingWithTupleField(
                                    place.identifier.to_owned(),
                                    *index,
                                ));
                            }
                            PlaceElement::StructureField(identifier) => identifier,
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
        self.variables
            .insert(name, Variable::new(value, is_mutable));
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

    pub fn declare_type(&mut self, name: String, type_variant: TypeVariant) -> Result<(), Error> {
        if self.is_type_declared(&name) {
            return Err(Error::RedeclaredType(name));
        }
        self.types.insert(name, type_variant);
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

    fn is_type_declared(&self, name: &str) -> bool {
        if self.types.contains_key(name) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_type_declared(name),
                None => false,
            }
        }
    }
}
