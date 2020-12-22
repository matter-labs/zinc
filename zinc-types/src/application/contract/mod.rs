//!
//! The bytecode contract application.
//!

pub mod method;

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::application::unit_test::UnitTest;
use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::instructions::Instruction;

use self::method::Method;

///
/// The bytecode contract application.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: String,
    /// The contract storage structure.
    pub storage: Vec<ContractFieldType>,
    /// The contract methods.
    pub methods: HashMap<String, Method>,
    /// The contract unit tests.
    pub unit_tests: HashMap<String, UnitTest>,
    /// The contract bytecode instructions.
    pub instructions: Vec<Instruction>,
}

impl Contract {
    ///
    /// Creates a contract application instance.
    ///
    pub fn new(
        name: String,
        storage: Vec<ContractFieldType>,
        methods: HashMap<String, Method>,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            name,
            storage,
            methods,
            unit_tests,
            instructions,
        }
    }
}
