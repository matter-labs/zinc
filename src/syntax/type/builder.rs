//!
//! The syntax type builder.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::syntax::Identifier;
use crate::syntax::Type;
use crate::syntax::TypeKeyword;

#[derive(Default)]
pub struct Builder {
    keyword: Option<TypeKeyword>,
    identifier: Option<Identifier>,
    fields: Option<Vec<(Identifier, Type)>>,
    variants: Option<Vec<Identifier>>,
    elements: Option<Vec<Type>>,
    generic_type: Option<Type>,
    vector_size: Option<usize>,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "missing struct identifier")]
    MissingStructIdentifier,
    #[fail(display = "missing enum identifier")]
    MissingEnumIdentifier,
    #[fail(display = "missing memory vector generic type")]
    MissingMemoryVectorGenericType,
    #[fail(display = "missing memory vector size")]
    MissingMemoryVectorSize,
    #[fail(display = "missing storage vector generic type")]
    MissingStorageVectorGenericType,
    #[fail(display = "missing storage vector size")]
    MissingStorageVectorSize,
}

impl Builder {
    pub fn set_keyword(&mut self, value: TypeKeyword) {
        self.keyword = Some(value);
    }

    //    pub fn set_identifier(&mut self, value: Identifier) {
    //        self.identifier = Some(value);
    //    }
    //
    //    pub fn add_field(&mut self, identifier: Identifier, r#type: Type) {
    //        match self.fields {
    //            Some(ref mut fields) => fields.push((identifier, r#type)),
    //            None => self.fields = Some(vec![]),
    //        }
    //    }
    //
    //    pub fn add_variant(&mut self, identifier: Identifier) {
    //        match self.variants {
    //            Some(ref mut variants) => variants.push(identifier),
    //            None => self.variants = Some(vec![]),
    //        }
    //    }
    //
    //    pub fn add_element(&mut self, r#type: Type) {
    //        match self.elements {
    //            Some(ref mut elements) => elements.push(r#type),
    //            None => self.elements = Some(vec![]),
    //        }
    //    }
    //
    //    pub fn set_generic_type(&mut self, value: Type) {
    //        self.generic_type = Some(value);
    //    }
    //
    //    pub fn set_vector_size(&mut self, value: usize) {
    //        self.vector_size = Some(value);
    //    }

    pub fn finish(self) -> Result<Type, Error> {
        match self.keyword {
            None => match self.elements {
                Some(elements) => Ok(Type::Tuple(elements)),
                None => Ok(Type::Void),
            },
            Some(TypeKeyword::Uint(bitlength)) => Ok(Type::Uint(bitlength)),
            Some(TypeKeyword::Int(bitlength)) => Ok(Type::Int(bitlength)),
            Some(TypeKeyword::Field) => Ok(Type::Field),
            Some(TypeKeyword::Bool) => Ok(Type::Bool),
            Some(TypeKeyword::Struct) => match self.identifier {
                Some(identiifer) => Ok(Type::Struct(identiifer, self.fields.unwrap_or_default())),
                None => Err(Error::MissingStructIdentifier),
            },
            Some(TypeKeyword::Enum) => match self.identifier {
                Some(identiifer) => Ok(Type::Enum(identiifer, self.variants.unwrap_or_default())),
                None => Err(Error::MissingEnumIdentifier),
            },
            Some(TypeKeyword::MemoryVector) => {
                let generic_type = match self.generic_type {
                    Some(generic_type) => generic_type,
                    None => return Err(Error::MissingMemoryVectorGenericType),
                };

                let vector_size = match self.vector_size {
                    Some(vector_size) => vector_size,
                    None => return Err(Error::MissingMemoryVectorSize),
                };

                Ok(Type::MemoryVector(Box::new(generic_type), vector_size))
            }
            Some(TypeKeyword::StorageVector) => {
                let generic_type = match self.generic_type {
                    Some(generic_type) => generic_type,
                    None => return Err(Error::MissingStorageVectorGenericType),
                };

                let vector_size = match self.vector_size {
                    Some(vector_size) => vector_size,
                    None => return Err(Error::MissingStorageVectorSize),
                };

                Ok(Type::StorageVector(Box::new(generic_type), vector_size))
            }
        }
    }
}
