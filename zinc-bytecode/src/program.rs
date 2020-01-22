use crate::Instruction;
use serde_derive::{Serialize, Deserialize};

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
    Struct(Vec<DataType>),
    Array(Box<DataType>, usize),
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub input: DataType,
    pub output: DataType,
    pub bytecode: Vec<Instruction>,
}

impl Program {
    pub fn new(input: DataType, output: DataType, bytecode: &[Instruction]) -> Self {
        Self {
            input,
            output,
            bytecode: bytecode.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Failed to serialize program")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| format!("{:?}", e))
    }
}
