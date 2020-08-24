//!
//! The Zinc VM bytecode circuit program.
//!

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as BuildType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

///
/// The circuit program.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    /// The circuit name.
    pub name: String,
    /// The circuit entry address.
    pub address: usize,
    /// The circuit entry input arguments structure type.
    pub input: BuildType,
    /// The circuit entry output type.
    pub output: BuildType,
    /// The circuit unit tests.
    pub unit_tests: HashMap<String, UnitTest>,
    /// The circuit bytecode instructions.
    pub instructions: Vec<Instruction>,
}

impl Circuit {
    ///
    /// Creates a circuit program instance.
    ///
    pub fn new(
        name: String,
        address: usize,
        input: BuildType,
        output: BuildType,
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
