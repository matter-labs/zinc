//!
//! The Zinc VM template type.
//!

pub mod scalar;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::scalar::Type as ScalarType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    Unit,
    Scalar(ScalarType),
    // Enum is always a field
    Enum,

    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Struct(Vec<(String, Type)>),
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Scalar(_) => 1,
            Self::Enum => 1,

            Self::Array(r#type, size) => r#type.size() * *size,
            Self::Tuple(fields) => fields.iter().map(|r#type| r#type.size()).sum(),
            Self::Struct(fields) => fields.iter().map(|(_, r#type)| r#type.size()).sum(),
        }
    }

    pub fn to_scalar_types(&self) -> Vec<ScalarType> {
        fn internal(types: &mut Vec<ScalarType>, dtype: &Type) {
            match dtype {
                Type::Unit => {}
                Type::Scalar(scalar_type) => {
                    types.push(scalar_type.to_owned());
                }
                Type::Enum => {
                    types.push(ScalarType::Field);
                }
                Type::Struct(fields) => {
                    for (_, r#type) in fields {
                        internal(types, r#type);
                    }
                }
                Type::Tuple(fields) => {
                    for r#type in fields {
                        internal(types, r#type);
                    }
                }
                Type::Array(r#type, size) => {
                    for _ in 0..*size {
                        internal(types, r#type.as_ref());
                    }
                }
            }
        }

        let mut types = Vec::new();
        internal(&mut types, self);
        types
    }
}
