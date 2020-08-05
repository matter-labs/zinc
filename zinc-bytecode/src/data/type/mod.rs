//!
//! The Zinc VM type.
//!

pub mod scalar;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::scalar::Type as ScalarType;

///
/// The bytecode metadata type.
///
/// Is converted from the bytecode generator type during writing the metadata.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    /// The `()` type.
    Unit,
    /// See the inner element description.
    Scalar(ScalarType),
    /// The `enum` type, which is treated as `field` for now. // TODO: enum validation
    Enumeration,

    /// The array type.
    Array(Box<Type>, usize),
    /// The tuple type.
    Tuple(Vec<Type>),
    /// The structure type.
    Structure(Vec<(String, Type)>),
    /// The contract type, which behaves almost like a structure, but its size is zero.
    /// The zero size prevents the `self` contract alias to be considered as a part of contract
    /// entry input, since `self` is used to address the contract storage, but not the entry input.
    Contract(Vec<(String, Type)>),
}

impl Type {
    ///
    /// Creates an empty unit test input instance.
    ///
    pub fn new_empty_structure() -> Self {
        Self::Structure(vec![])
    }

    ///
    /// Skips the contract values since they are not supposed to be passed as the metadata.
    ///
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

    ///
    /// Wraps the type into a structure, which consists of the contract output result itself and a
    /// field for the contract storage root hash, which is also an implicit part of the entry output.
    ///
    pub fn into_contract_metadata(self) -> Self {
        Self::Structure(vec![
            ("$result".to_owned(), self),
            ("$root_hash".to_owned(), Self::Scalar(ScalarType::Field)),
        ])
    }

    ///
    /// Returns the type size.
    ///
    /// Skips the contract values since they are not supposed to be passed from the entry input file,
    /// but are read from the contract storage.
    ///
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
