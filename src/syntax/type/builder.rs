//!
//! The syntax type builder.
//!

use crate::lexical::Keyword;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    name: Option<Keyword>,
    //    identifier: Option<Identifier>,
    //    fields: Option<Vec<(Identifier, Type)>>,
    //    variants: Option<Vec<Identifier>>,
    //    elements: Option<Vec<Type>>,
    //    generic_type: Option<Type>,
    //    vector_size: Option<usize>,
}

impl Builder {
    pub fn set_name(&mut self, value: Keyword) {
        self.name = Some(value);
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

    pub fn finish(self) -> Type {
        match self.name {
            //            None => match self.elements {
            //                Some(elements) => Ok(Type::Tuple(elements)),
            //                None => Ok(Type::Void),
            //            },
            Some(Keyword::Field) => Type::Field,
            Some(Keyword::Uint(bitlength)) => Type::Uint { bitlength },
            Some(Keyword::Int(bitlength)) => Type::Int { bitlength },
            Some(Keyword::Bool) => Type::Bool,
            //            Some(Keyword::Struct) => match self.identifier {
            //                Some(identiifer) => Ok(Type::Struct(identiifer, self.fields.unwrap_or_default())),
            //                None => Err(Error::MissingStructIdentifier),
            //            },
            //            Some(Keyword::Enum) => match self.identifier {
            //                Some(identiifer) => Ok(Type::Enum(identiifer, self.variants.unwrap_or_default())),
            //                None => Err(Error::MissingEnumIdentifier),
            //            },
            //            Some(Keyword::MemoryVector) => {
            //                let generic_type = match self.generic_type {
            //                    Some(generic_type) => generic_type,
            //                    None => return Err(Error::MissingMemoryVectorGenericType),
            //                };
            //
            //                let vector_size = match self.vector_size {
            //                    Some(vector_size) => vector_size,
            //                    None => return Err(Error::MissingMemoryVectorSize),
            //                };
            //
            //                Ok(Type::MemoryVector(Box::new(generic_type), vector_size))
            //            }
            //            Some(Keyword::StorageVector) => {
            //                let generic_type = match self.generic_type {
            //                    Some(generic_type) => generic_type,
            //                    None => return Err(Error::MissingStorageVectorGenericType),
            //                };
            //
            //                let vector_size = match self.vector_size {
            //                    Some(vector_size) => vector_size,
            //                    None => return Err(Error::MissingStorageVectorSize),
            //                };
            //
            //                Ok(Type::StorageVector(Box::new(generic_type), vector_size))
            //            }
            _ => unimplemented!(),
        }
    }
}
