//!
//! The bytecode library.
//!

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::application::unit_test::UnitTest;
use crate::instructions::Instruction;

///
/// The bytecode library.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// The library name.
    pub name: String,
    /// The library unit tests.
    pub unit_tests: HashMap<String, UnitTest>,
    /// The library bytecode instructions.
    pub instructions: Vec<Instruction>,
}

impl Library {
    ///
    /// Creates a library instance.
    ///
    pub fn new(
        name: String,
        unit_tests: HashMap<String, UnitTest>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            name,
            unit_tests,
            instructions,
        }
    }
}
