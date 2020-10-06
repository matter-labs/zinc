//!
//! The Zinc VM bytecode application.
//!

pub mod circuit;
pub mod contract;
pub mod unit_test;

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use crate::application::unit_test::UnitTest;
use crate::bytes::Bytes;
use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::data::r#type::Type;
use crate::data::value::Value;
use crate::instructions::Instruction;

use self::circuit::Circuit;
use self::contract::method::Method as ContractMethod;
use self::contract::Contract;

///
/// The Zinc application.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Application {
    /// The circuit application variant.
    Circuit(Circuit),
    /// The contract application variant.
    Contract(Contract),
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
    /// Returns the application instructions reference.
    ///
    pub fn instructions(&self) -> &[Instruction] {
        match self {
            Self::Circuit(ref inner) => inner.instructions.as_slice(),
            Self::Contract(ref inner) => inner.instructions.as_slice(),
        }
    }

    ///
    /// Converts the compiled application state into a set of byte arrays, which are ready to be
    /// written to the Zinc project build files.
    ///
    pub fn into_bytes(self) -> Bytes {
        match self {
            Application::Circuit(circuit) => {
                let input_template =
                    serde_json::to_vec_pretty(&Value::new(circuit.input.clone()).into_json())
                        .expect(zinc_const::panic::DATA_CONVERSION);
                let output_template =
                    serde_json::to_vec_pretty(&Value::new(circuit.output.clone()).into_json())
                        .expect(zinc_const::panic::DATA_CONVERSION);

                let bytecode = Application::Circuit(circuit).into_vec();

                Bytes::new_circuit(bytecode, input_template, output_template)
            }
            Application::Contract(contract) => {
                let mut input_templates = HashMap::with_capacity(contract.methods.len());
                let mut output_templates = HashMap::with_capacity(contract.methods.len());
                for (name, method) in contract.methods.iter() {
                    input_templates.insert(
                        name.to_owned(),
                        serde_json::to_vec_pretty(&Value::new(method.input.to_owned()).into_json())
                            .expect(zinc_const::panic::DATA_CONVERSION),
                    );
                    output_templates.insert(
                        name.to_owned(),
                        serde_json::to_vec_pretty(
                            &Value::new(method.output.to_owned()).into_json(),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    );
                }

                let fields: Vec<JsonValue> = contract
                    .storage
                    .clone()
                    .into_iter()
                    .map(|field| Value::new(field.r#type).into_json())
                    .collect();
                let storage = serde_json::to_vec_pretty(&JsonValue::Array(fields))
                    .expect(zinc_const::panic::DATA_CONVERSION);

                let bytecode = Application::Contract(contract).into_vec();

                Bytes::new_contract(bytecode, storage, input_templates, output_templates)
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
