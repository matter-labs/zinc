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
    Structure(Vec<(String, Type)>),
}

impl Type {
    pub fn into_flat_scalar_types(self) -> Vec<ScalarType> {
        match self {
            Type::Unit => vec![],
            Type::Scalar(scalar_type) => vec![scalar_type],
            Type::Enum => vec![ScalarType::Field],
            Type::Array(r#type, size) => vec![Self::into_flat_scalar_types(*r#type); size]
                .into_iter()
                .flatten()
                .collect(),
            Type::Tuple(types) => types
                .into_iter()
                .map(Self::into_flat_scalar_types)
                .flatten()
                .collect(),
            Type::Structure(types) => types
                .into_iter()
                .map(|(_name, r#type)| Self::into_flat_scalar_types(r#type))
                .flatten()
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Scalar(_) => 1,
            Self::Enum => 1,

            Self::Array(r#type, size) => r#type.size() * *size,
            Self::Tuple(fields) => fields.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure(fields) => fields.iter().map(|(_, r#type)| r#type.size()).sum(),
        }
    }
}
