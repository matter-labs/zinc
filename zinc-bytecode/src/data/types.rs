use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntegerType {
    pub is_signed: bool,
    pub bit_length: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ScalarType {
    Field,
    Boolean,
    Integer(IntegerType),
}

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
