//!
//! The bytecode application.
//!

pub mod circuit;
pub mod contract;
pub mod library;
pub mod unit_test;

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::application::unit_test::UnitTest;
use crate::build::input::Input as InputBuild;
use crate::build::Build;
use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::data::r#type::Type;
use crate::data::value::Value;
use crate::instructions::Instruction;

use self::circuit::Circuit;
use self::contract::method::Method as ContractMethod;
use self::contract::Contract;
use self::library::Library;

///
/// The bytecode application.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Application {
    /// The circuit application variant.
    Circuit(Circuit),
    /// The contract application variant.
    Contract(Contract),
    /// The library variant.
    Library(Library),
}

impl Application {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_circuit(
        name: String,
        address: usize,
        input: Type,
        output: Type,
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
        storage: Vec<ContractFieldType>,
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
    /// A shortcut constructor.
    ///
    pub fn new_library(
        name: String,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self::Library(Library::new(name, unit_tests, instructions))
    }

    ///
    /// Returns the application instructions reference.
    ///
    pub fn instructions(&self) -> &[Instruction] {
        match self {
            Self::Circuit(ref inner) => inner.instructions.as_slice(),
            Self::Contract(ref inner) => inner.instructions.as_slice(),
            Self::Library(ref inner) => inner.instructions.as_slice(),
        }
    }

    ///
    /// Converts the compiled application state into a set of byte arrays, which are ready to be
    /// written to the Zinc project build files.
    ///
    pub fn into_build(self) -> Build {
        match self {
            Application::Circuit(circuit) => {
                let arguments = Value::new(circuit.input.clone()).into_json();
                let bytecode = Application::Circuit(circuit).into_vec();

                Build::new(bytecode, InputBuild::new_circuit(arguments))
            }
            Application::Contract(contract) => {
                let mut arguments = HashMap::with_capacity(contract.methods.len());
                for (name, method) in contract.methods.iter() {
                    arguments.insert(
                        name.to_owned(),
                        Value::new(method.input.to_owned()).into_json(),
                    );
                }

                let fields: Vec<serde_json::Value> = contract
                    .storage
                    .clone()
                    .into_iter()
                    .map(|field| Value::new(field.r#type).into_json())
                    .collect();
                let mut storages = HashMap::with_capacity(1);
                storages.insert(
                    "0x0000000000000000000000000000000000000000".to_owned(),
                    serde_json::Value::Array(fields),
                );

                let transaction = serde_json::json!({
                    "sender": "0x0000000000000000000000000000000000000000",
                    "recipient": "0x0000000000000000000000000000000000000000",
                    "token_address": "0x0000000000000000000000000000000000000000",
                    "amount": "0",
                });

                let bytecode = Application::Contract(contract).into_vec();

                Build::new(
                    bytecode,
                    InputBuild::new_contract(storages, transaction, arguments),
                )
            }
            Application::Library(library) => {
                let bytecode = Application::Library(library).into_vec();

                Build::new(bytecode, InputBuild::new_library())
            }
        }
    }

    ///
    /// Deserializes an application from the byte `slice`.
    ///
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, String> {
        bincode::deserialize(slice).map_err(|error| format!("{:?}", error))
    }

    ///
    /// Serializes the application to a byte array.
    ///
    pub fn into_vec(self) -> Vec<u8> {
        bincode::serialize(&self).expect(zinc_const::panic::DATA_CONVERSION)
    }
}
