//!
//! The Zinc VM bytecode program.
//!

pub mod circuit;
pub mod contract;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instruction::Instruction;

use self::circuit::Circuit;
use self::contract::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Program {
    Circuit(Circuit),
    Contract(Contract),
}

impl Program {
    pub fn new_circuit(input: DataType, output: DataType, instructions: Vec<Instruction>) -> Self {
        Self::Circuit(Circuit::new(input, output, instructions))
    }

    pub fn new_contract(
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
    ) -> Self {
        Self::Contract(Contract::new(input, output, instructions, storage))
    }

    pub fn input(&self) -> &DataType {
        match self {
            Self::Circuit(ref inner) => &inner.input,
            Self::Contract(ref inner) => &inner.input,
        }
    }

    pub fn output(&self) -> &DataType {
        match self {
            Self::Circuit(ref inner) => &inner.output,
            Self::Contract(ref inner) => &inner.output,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        match self {
            Self::Circuit(ref inner) => inner.instructions.as_slice(),
            Self::Contract(ref inner) => inner.instructions.as_slice(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| format!("{:?}", e))
    }

    pub fn into_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).expect("Failed to serialize program")
    }
}
