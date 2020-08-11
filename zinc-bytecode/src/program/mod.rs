//!
//! The Zinc VM bytecode program.
//!

pub mod circuit;
pub mod contract;

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;

use self::circuit::unit_test::UnitTest as CircuitUnitTest;
use self::circuit::Circuit;
use self::contract::method::Method as ContractMethod;
use self::contract::unit_test::UnitTest as ContractUnitTest;
use self::contract::Contract;

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
    /// A shortcut constructor.
    ///
    pub fn new_circuit(
        name: String,
        instructions: Vec<Instruction>,
        input: DataType,
        output: DataType,
        unit_tests: HashMap<String, CircuitUnitTest>,
    ) -> Self {
        Self::Circuit(Circuit::new(name, instructions, input, output, unit_tests))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_contract(
        name: String,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
        methods: HashMap<String, ContractMethod>,
        unit_tests: HashMap<String, ContractUnitTest>,
    ) -> Self {
        Self::Contract(Contract::new(
            name,
            instructions,
            storage,
            methods,
            unit_tests,
        ))
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
