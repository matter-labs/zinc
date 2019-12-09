//!
//! The semantic analyzer scope.
//!

mod error;
mod item;

pub use self::error::Error;
pub use self::item::Item;
pub use self::item::Static as StaticItem;
pub use self::item::Variable as VariableItem;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::semantic::Constant;
use crate::semantic::Place;
use crate::semantic::PlaceDescriptor;
use crate::semantic::Type;
use crate::semantic::Type as TypeItem;

#[derive(Debug, Default, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
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
        variable: VariableItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Variable(variable));
        Ok(())
    }

    pub fn declare_constant(
        &mut self,
        identifier: String,
        constant: Constant,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Constant(constant));
        Ok(())
    }

    pub fn declare_static(
        &mut self,
        identifier: String,
        r#static: StaticItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Static(r#static));
        Ok(())
    }

    pub fn declare_type(&mut self, identifier: String, r#type: TypeItem) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Type(r#type));
        Ok(())
    }

    pub fn declare_module(&mut self, identifier: String, scope: Scope) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Module(Rc::new(scope)));
        Ok(())
    }

    pub fn get_variable(&self, place: &Place) -> Result<VariableItem, Error> {
        match self.items.get(&place.identifier) {
            Some(Item::Variable(variable)) => {
                let mut variable = variable.to_owned();
                for descriptor in place.descriptors.iter() {
                    match (descriptor, &variable.r#type) {
                        (
                            PlaceDescriptor::ArrayIndex(array_index),
                            Type::Array {
                                r#type: array_element_type,
                                size: array_size,
                            },
                        ) => {
                            let array_size = *array_size;
                            let array_index = *array_index;
                            if array_index >= array_size {
                                return Err(Error::ArrayIndexOutOfRange(
                                    array_index,
                                    variable.r#type.to_owned(),
                                ));
                            }
                            variable.address += array_index * array_element_type.size();
                            variable.r#type = *array_element_type.to_owned();
                        }
                        (
                            PlaceDescriptor::TupleField(tuple_field),
                            Type::Tuple { types: tuple_types },
                        ) => {
                            let tuple_field = *tuple_field;
                            if tuple_field >= tuple_types.len() {
                                return Err(Error::TupleFieldDoesNotExist(
                                    tuple_field,
                                    variable.r#type.to_owned(),
                                ));
                            }
                            for _tuple_field_index in 0..tuple_field {
                                variable.address += tuple_types[tuple_field].size();
                            }
                            variable.r#type = tuple_types[tuple_field].to_owned();
                        }
                        (
                            PlaceDescriptor::StructureField(structure_field),
                            Type::Structure { fields, .. },
                        ) => {
                            let mut found_type = None;
                            for (field_name, field_type) in fields.iter() {
                                if field_name == structure_field {
                                    found_type = Some(field_type);
                                    break;
                                }
                                variable.address += field_type.size();
                            }
                            match found_type.take() {
                                Some(found_type) => variable.r#type = found_type.to_owned(),
                                None => {
                                    return Err(Error::StructureFieldDoesNotExist(
                                        structure_field.to_owned(),
                                        variable.r#type.to_owned(),
                                    ))
                                }
                            }
                        }
                        (descriptor, inner_type) => {
                            return Err(Error::InvalidDescriptor(
                                inner_type.to_owned(),
                                descriptor.to_owned(),
                            ))
                        }
                    }
                }

                Ok(variable)
            }
            Some(_item) => Err(Error::ItemIsNotVariable(place.identifier.to_owned())),
            None => match self.parent {
                Some(ref parent) => parent.borrow().get_variable(place),
                None => Err(Error::ItemUndeclared(place.identifier.to_owned())),
            },
        }
    }

    pub fn get_item(&self, identifier: &str) -> Result<Item, Error> {
        match self.items.get(identifier) {
            Some(item) => Ok(item.to_owned()),
            None => match self.parent {
                Some(ref parent) => parent.borrow().get_item(identifier),
                None => Err(Error::ItemUndeclared(identifier.to_owned())),
            },
        }
    }

    pub fn get_variant(&self, enumeration: &str, variant: &str) -> Result<usize, Error> {
        match self.items.get(enumeration) {
            Some(Item::Type(Type::Enumeration { variants, .. })) => {
                for (name, value) in variants.iter() {
                    if name == variant {
                        return Ok(*value);
                    }
                }
                Err(Error::EnumerationVariantDoesNotExist(
                    variant.to_owned(),
                    enumeration.to_owned(),
                ))
            }
            Some(_item) => Err(Error::ItemIsNotEnumeration(enumeration.to_owned())),
            None => match self.parent {
                Some(ref parent) => parent.borrow().get_variant(enumeration, variant),
                None => Err(Error::ItemUndeclared(enumeration.to_owned())),
            },
        }
    }

    pub fn is_item_declared(&self, identifier: &str) -> bool {
        if self.items.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_item_declared(identifier),
                None => false,
            }
        }
    }
}
