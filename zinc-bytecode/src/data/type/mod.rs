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
    Enumeration,

    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Structure(Vec<(String, Type)>),
    Contract(Vec<(String, Type)>),
}

impl Type {
    pub fn new_empty_structure() -> Self {
        Self::Structure(vec![])
    }

    pub fn into_flat_scalar_types(self) -> Vec<ScalarType> {
        match self {
            Self::Unit => vec![],
            Self::Scalar(scalar_type) => vec![scalar_type],
            Self::Enumeration => vec![ScalarType::Field],

            Self::Array(r#type, size) => vec![Self::into_flat_scalar_types(*r#type); size]
                .into_iter()
                .flatten()
                .collect(),
            Self::Tuple(types) => types
                .into_iter()
                .map(Self::into_flat_scalar_types)
                .flatten()
                .collect(),
            Self::Structure(types) => types
                .into_iter()
                .map(|(_name, r#type)| Self::into_flat_scalar_types(r#type))
                .flatten()
                .collect(),
            Self::Contract(_) => vec![],
        }
    }

    pub fn into_contract_metadata(self) -> Self {
        Self::Structure(vec![
            ("$result".to_owned(), self),
            ("$root_hash".to_owned(), Self::Scalar(ScalarType::Field)),
        ])
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Scalar(_) => 1,
            Self::Enumeration => 1,

            Self::Array(r#type, size) => r#type.size() * *size,
            Self::Tuple(fields) => fields.iter().map(Self::size).sum(),
            Self::Structure(fields) => fields.iter().map(|(_, r#type)| r#type.size()).sum(),
            Self::Contract(_) => 0,
        }
    }
}
