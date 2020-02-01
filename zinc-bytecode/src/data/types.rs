use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BinaryInteger {
    pub is_signed: bool,
    pub bit_length: usize,
}

#[derive(Serialize, Deserialize)]
pub enum PrimitiveType {
    Field,
    Integer(BinaryInteger),
}

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Unit,
    Primitive(PrimitiveType),
    // Enum is always a field
    Enum,
    Struct(Vec<(String, DataType)>),
    Tuple(Vec<DataType>),
    Array(Box<DataType>, usize),
}
