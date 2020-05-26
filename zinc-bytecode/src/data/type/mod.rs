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
            Type::Unit => 0,
            Type::Scalar(_) => 1,
            Type::Enum => 1,

            Type::Array(r#type, size) => r#type.size() * *size,
            Type::Tuple(fields) => fields.iter().map(|r#type| r#type.size()).sum(),
            Type::Struct(fields) => fields.iter().map(|(_, r#type)| r#type.size()).sum(),
        }
    }
}
