//!
//! The scalar type.
//!

pub mod integer;

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use self::integer::Type as IntegerType;

///
/// The scalar type.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    /// The `bool` scalar type.
    Boolean,
    /// The `u{n}` or `i{n}` scalar type.
    Integer(IntegerType),
    /// The `field` scalar type.
    Field,
}

impl Type {
    ///
    /// Creates the ETH address type.
    ///
    pub fn eth_address() -> Self {
        Self::Integer(IntegerType::ETH_ADDRESS)
    }

    ///
    /// Checks whether the type is a signed integer.
    ///
    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Type::Integer(IntegerType {
                is_signed: true, ..
            })
        )
    }
}

impl From<IntegerType> for Type {
    fn from(inner: IntegerType) -> Self {
        Type::Integer(inner)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Field => write!(f, "field"),
        }
    }
}
