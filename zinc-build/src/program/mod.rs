//!
//! The Zinc VM bytecode program.
//!

pub mod circuit;
pub mod contract;
pub mod unit_test;

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as BuildType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

use self::circuit::Circuit;
use self::contract::method::Method as ContractMethod;
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
        address: usize,
        input: BuildType,
        output: BuildType,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self::Circuit(Circuit::new(
            name,
            address,
            input,
            output,
            unit_tests,
            instructions,
        ))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_contract(
        name: String,
        storage: Vec<(String, BuildType)>,
        methods: HashMap<String, ContractMethod>,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self::Contract(Contract::new(
            name,
            storage,
            methods,
            unit_tests,
            instructions,
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
