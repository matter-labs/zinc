use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
    pub is_signed: bool,
    pub bit_length: usize,
}

#[derive(Serialize, Deserialize)]
pub enum ScalarType {
    Field,
    Boolean,
    Integer(IntegerType),
}

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Unit,
    Scalar(ScalarType),
    // Enum is always a field
    Enum,
    Struct(Vec<(String, DataType)>),
    Tuple(Vec<DataType>),
    Array(Box<DataType>, usize),
}
