//!
//! The Zinc VM scalar type.
//!

pub mod integer;

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::integer::Type as IntegerType;

///
/// The Zinc VM scalar type.
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
