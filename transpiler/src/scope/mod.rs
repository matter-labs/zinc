//!
//! The transpiler scope.
//!

mod error;
mod variable;

pub use self::error::Error;
pub use self::variable::Variable;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use parser::TypeVariant;

use crate::element::Descriptor;

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

    pub fn get_variable(
        &self,
        identifier: &str,
        descriptors: Vec<Descriptor>,
    ) -> Result<Variable, Error> {
        if let Some(variable) = self.variables.get(identifier) {
            let mut type_variant = &variable.type_variant;
            for descriptor in descriptors.iter() {
                type_variant = match type_variant {
                    TypeVariant::Array { type_variant, size } => {
                        let index = match descriptor {
                            Descriptor::Array(index) => *index,
                            Descriptor::Tuple(index) => {
                                return Err(Error::AddressArrayAsTuple(
                                    identifier.to_owned(),
                                    *index,
                                ));
                            }
                            Descriptor::Structure(field) => {
                                return Err(Error::AddressArrayAsStructure(
                                    identifier.to_owned(),
                                    field.to_owned(),
                                ));
                            }
                        };

                        if index >= *size {
                            return Err(Error::ArrayIndexOutOfRange(index, identifier.to_owned()));
                        }

                        &*type_variant
                    }
                    TypeVariant::Tuple { type_variants } => {
                        let field = match descriptor {
                            Descriptor::Array(index) => {
                                return Err(Error::AccessTupleAsArray(
                                    identifier.to_owned(),
                                    *index,
                                ));
                            }
                            Descriptor::Tuple(field) => *field,
                            Descriptor::Structure(field) => {
                                return Err(Error::AccessTupleAsStructure(
                                    identifier.to_owned(),
                                    field.to_owned(),
                                ));
                            }
                        };

                        if field >= type_variants.len() {
                            return Err(Error::TupleFieldNotExists(field, identifier.to_owned()));
                        }

                        &type_variants[0]
                    }
                    TypeVariant::Structure { fields, .. } => {
                        let field = match descriptor {
                            Descriptor::Array(index) => {
                                return Err(Error::AccessStructureAsArray(
                                    identifier.to_owned(),
                                    *index,
                                ));
                            }
                            Descriptor::Tuple(index) => {
                                return Err(Error::AccessStructureAsTuple(
                                    identifier.to_owned(),
                                    *index,
                                ));
                            }
                            Descriptor::Structure(identifier) => identifier,
                        };

                        fields.get(identifier).ok_or_else(|| {
                            Error::StructureFieldNotExists(field.to_owned(), identifier.to_owned())
                        })?
                    }
                    _ => {
                        return Err(Error::AddressingPrimitiveTypeVariable(
                            identifier.to_owned(),
                        ))
                    }
                };
            }
            Ok(Variable::new(type_variant.to_owned(), variable.is_mutable))
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().get_variable(identifier, descriptors),
                None => Err(Error::UndeclaredVariable(identifier.to_owned())),
            }
        }
    }

    pub fn declare_variable(
        &mut self,
        identifier: String,
        type_variant: TypeVariant,
        is_mutable: bool,
    ) -> Result<(), Error> {
        if self.is_variable_declared(&identifier) {
            return Err(Error::RedeclaredVariable(identifier));
        }
        self.variables
            .insert(identifier, Variable::new(type_variant, is_mutable));
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

    pub fn declare_type(
        &mut self,
        identifier: String,
        type_variant: TypeVariant,
    ) -> Result<(), Error> {
        if self.is_type_declared(&identifier) {
            return Err(Error::RedeclaredType(identifier));
        }
        self.types.insert(identifier, type_variant);
        Ok(())
    }

    fn is_variable_declared(&self, identifier: &str) -> bool {
        if self.variables.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_variable_declared(identifier),
                None => false,
            }
        }
    }

    fn is_type_declared(&self, identifier: &str) -> bool {
        if self.types.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_type_declared(identifier),
                None => false,
            }
        }
    }
}
