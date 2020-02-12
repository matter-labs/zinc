pub use crate::scalar::{IntegerType, ScalarType};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DataType {
    Unit,
    Scalar(ScalarType),
    // Enum is always a field
    Enum,
    Struct(Vec<(String, DataType)>),
    Tuple(Vec<DataType>),
    Array(Box<DataType>, usize),
}

impl DataType {
    pub fn size(&self) -> usize {
        match self {
            DataType::Unit => 0,
            DataType::Scalar(_) => 1,
            DataType::Enum => 1,
            DataType::Struct(fields) => fields.iter().map(|(_, f)| f.size()).sum(),
            DataType::Tuple(fields) => fields.iter().map(|f| f.size()).sum(),
            DataType::Array(element_type, array_size) => element_type.size() * *array_size,
        }
    }
}
