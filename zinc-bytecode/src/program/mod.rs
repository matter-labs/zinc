//!
//! The Zinc VM bytecode program.
//!

pub mod circuit;
pub mod contract;
pub mod unit_test;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;

use self::circuit::Circuit;
use self::contract::Contract;
use self::unit_test::UnitTest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Program {
    Circuit(Circuit),
    Contract(Contract),
}

impl Program {
    pub fn new_circuit(input: DataType, output: DataType, instructions: Vec<Instruction>) -> Self {
        Self::Circuit(Circuit::new(input, output, instructions, None))
    }

    pub fn new_circuit_unit_test(instructions: Vec<Instruction>, unit_test: UnitTest) -> Self {
        Self::Circuit(Circuit::new(
            DataType::new_empty_structure(),
            DataType::Unit,
            instructions,
            Some(unit_test),
        ))
    }

    pub fn new_contract(
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
    ) -> Self {
        Self::Contract(Contract::new(
            input,
            output.into_contract_metadata(),
            instructions,
            storage,
            None,
        ))
    }

    pub fn new_contract_unit_test(
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
        unit_test: UnitTest,
    ) -> Self {
        Self::Contract(Contract::new(
            DataType::new_empty_structure(),
            DataType::Unit,
            instructions,
            storage,
            Some(unit_test),
        ))
    }

    pub fn input(&self) -> DataType {
        match self {
            Self::Circuit(ref inner) => inner.input.to_owned(),
            Self::Contract(ref inner) => inner.input.to_owned(),
        }
    }

    pub fn output(&self) -> DataType {
        match self {
            Self::Circuit(ref inner) => inner.output.to_owned(),
            Self::Contract(ref inner) => inner.output.to_owned(),
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
        bincode::serialize(&self).expect(crate::panic::BINARY_SERIALIZATION)
    }
}
