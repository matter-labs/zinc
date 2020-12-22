//!
//! The bytecode circuit application.
//!

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::application::unit_test::UnitTest;
use crate::data::r#type::Type;
use crate::instructions::Instruction;

///
/// The bytecode circuit application.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    /// The circuit name.
    pub name: String,
    /// The circuit entry address.
    pub address: usize,
    /// The circuit entry input arguments structure type.
    pub input: Type,
    /// The circuit entry output type.
    pub output: Type,
    /// The circuit unit tests.
    pub unit_tests: HashMap<String, UnitTest>,
    /// The circuit bytecode instructions.
    pub instructions: Vec<Instruction>,
}

impl Circuit {
    ///
    /// Creates a circuit application instance.
    ///
    pub fn new(
        name: String,
        address: usize,
        input: Type,
        output: Type,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            name,
            address,
            input,
            output,
            unit_tests,
            instructions,
        }
    }
}
