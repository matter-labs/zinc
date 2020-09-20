//!
//! The Zinc VM type.
//!

pub mod contract_field;
pub mod scalar;

use std::fmt;

use num_bigint::BigInt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::contract_field::ContractField;
use self::scalar::integer::Type as IntegerType;
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
    /// The `enum` type, which is specified in the input JSON file using variant name strings.
    Enumeration {
        /// The enumeration type bitlength.
        bitlength: usize,
        /// The variant list.
        variants: Vec<(String, BigInt)>,
    },

    /// The array type.
    Array(Box<Type>, usize),
    /// The tuple type.
    Tuple(Vec<Type>),
    /// The structure type.
    Structure(Vec<(String, Type)>),
    /// The contract type.
    Contract(Vec<ContractField>),
}

impl Type {
    ///
    /// Creates an empty unit test input instance.
    ///
    pub fn new_empty_structure() -> Self {
        Self::Structure(vec![])
    }

    ///
    /// Creates the ETH address type as integer.
    ///
    pub fn new_eth_address_integer() -> Self {
        Self::Scalar(ScalarType::Integer(IntegerType::ETH_ADDRESS))
    }

    ///
    /// Converts a complex type into an array of primitive scalar types, which is useful for
    /// reading an application input values.
    ///
    pub fn into_flat_scalar_types(self) -> Vec<ScalarType> {
        match self {
            Self::Unit => vec![],
            Self::Scalar(scalar_type) => vec![scalar_type],
            Self::Enumeration { bitlength, .. } => {
                vec![ScalarType::Integer(IntegerType::new(false, bitlength))]
            }

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
            Self::Contract(types) => types
                .into_iter()
                .map(|field| Self::into_flat_scalar_types(field.r#type))
                .flatten()
                .collect(),
        }
    }

    ///
    /// Wraps the type into a structure, which consists of the contract output result itself and a
    /// field for the contract storage root hash, which is also an implicit part of the method output.
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
    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Scalar(_) => 1,
            Self::Enumeration { .. } => 1,

            Self::Array(r#type, size) => r#type.size() * *size,
            Self::Tuple(fields) => fields.iter().map(Self::size).sum(),
            Self::Structure(fields) => fields.iter().map(|(_, r#type)| r#type.size()).sum(),
            Self::Contract(fields) => fields.iter().map(|field| field.r#type.size()).sum(),
        }
    }

    ///
    /// Removes the first structure field, if the field is a contract instance.
    ///
    /// Is used before passing through the input arguments of a contract method, where the first
    /// arguments is a contract instance, which is stored not in the data stack, but in the
    /// contract storage, and should not be taken into account when calculating the input size.
    ///
    pub fn remove_contract_instance(&mut self) {
        if let Self::Structure(fields) = self {
            if matches!(fields.first(), Some((_name, Self::Contract(_)))) {
                fields.remove(0);
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Scalar(inner) => write!(f, "{}", inner),
            Self::Enumeration { variants, .. } => write!(
                f,
                "enum {}",
                variants
                    .iter()
                    .map(|(name, value)| format!("{} = {}", name, value))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),

            Self::Array(inner, size) => write!(f, "[{}; {}]", inner, size),
            Self::Tuple(types) => write!(
                f,
                "({})",
                types
                    .iter()
                    .map(|r#type| r#type.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Structure(fields) => write!(
                f,
                "{}",
                fields
                    .iter()
                    .map(|(name, r#type)| format!("{}: {}", name, r#type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Contract(fields) => write!(
                f,
                "{}",
                fields
                    .iter()
                    .map(|field| format!("{}: {}", field.name, field.r#type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
