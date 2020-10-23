//!
//! The Zinc VM bytecode circuit application.
//!

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::application::unit_test::UnitTest;
use crate::data::r#type::Type as BuildType;
use crate::instructions::Instruction;

///
/// The circuit application.
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
    /// Creates a circuit application instance.
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
