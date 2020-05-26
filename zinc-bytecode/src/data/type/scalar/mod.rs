pub mod integer;

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::integer::Type as IntegerType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Boolean,
    Integer(IntegerType),
    Field,
}

impl Type {
    pub fn is_signed(&self) -> bool {
        match self {
            Type::Integer(IntegerType {
                is_signed: true, ..
            }) => true,
            _ => false,
        }
    }
}

impl From<IntegerType> for Type {
    fn from(inner: IntegerType) -> Self {
        Type::Integer(inner)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Field => write!(f, "field"),
        }
    }
}
