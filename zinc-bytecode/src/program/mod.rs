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

///
/// The Zinc program.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Program {
    /// The circuit program variant.
    Circuit(Circuit),
    /// The contract program variant.
    Contract(Contract),
}

impl Program {
    ///
    /// A shortcut constructor for a non-unit-test circuit.
    ///
    pub fn new_circuit(
        name: String,
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self::Circuit(Circuit::new(name, input, output, instructions, None))
    }

    ///
    /// A shortcut constructor for a unit-test circuit.
    ///
    pub fn new_circuit_unit_test(
        name: String,
        instructions: Vec<Instruction>,
        unit_test: UnitTest,
    ) -> Self {
        Self::Circuit(Circuit::new(
            name,
            DataType::new_empty_structure(),
            DataType::Unit,
            instructions,
            Some(unit_test),
        ))
    }

    ///
    /// A shortcut constructor for a non-unit-test contract.
    ///
    pub fn new_contract(
        name: String,
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
    ) -> Self {
        Self::Contract(Contract::new(
            name,
            input,
            output.into_contract_metadata(),
            instructions,
            storage,
            None,
        ))
    }

    ///
    /// A shortcut constructor for a unit-test contract.
    ///
    pub fn new_contract_unit_test(
        name: String,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
        unit_test: UnitTest,
    ) -> Self {
        Self::Contract(Contract::new(
            name,
            DataType::new_empty_structure(),
            DataType::Unit,
            instructions,
            storage,
            Some(unit_test),
        ))
    }

    ///
    /// Returns the program name.
    ///
    pub fn name(&self) -> String {
        match self {
            Self::Circuit(ref inner) => inner.name.to_owned(),
            Self::Contract(ref inner) => inner.name.to_owned(),
        }
    }

    ///
    /// Returns the program input.
    ///
    pub fn input(&self) -> DataType {
        match self {
            Self::Circuit(ref inner) => inner.input.to_owned(),
            Self::Contract(ref inner) => inner.input.to_owned(),
        }
    }

    ///
    /// Returns the program output.
    ///
    pub fn output(&self) -> DataType {
        match self {
            Self::Circuit(ref inner) => inner.output.to_owned(),
            Self::Contract(ref inner) => inner.output.to_owned(),
        }
    }

    ///
    /// Returns the program instructions reference.
    ///
    pub fn instructions(&self) -> &[Instruction] {
        match self {
            Self::Circuit(ref inner) => inner.instructions.as_slice(),
            Self::Contract(ref inner) => inner.instructions.as_slice(),
        }
    }

    ///
    /// Deserializes a program from `bytes`.
    ///
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes).map_err(|e| format!("{:?}", e))
    }

    ///
    /// Serializes the program into bytes.
    ///
    pub fn into_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).expect(zinc_const::panic::DATA_SERIALIZATION)
    }
}
