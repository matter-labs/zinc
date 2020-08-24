//!
//! The Zinc VM bytecode contract program.
//!

pub mod method;

use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as BuildType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

use self::method::Method;

///
/// The contract program.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: String,
    /// The contract storage structure.
    pub storage: Vec<(String, BuildType)>,
    /// The contract methods.
    pub methods: HashMap<String, Method>,
    /// The contract unit tests.
    pub unit_tests: HashMap<String, UnitTest>,
    /// The contract bytecode instructions.
    pub instructions: Vec<Instruction>,
}

impl Contract {
    ///
    /// Creates a contract program instance.
    ///
    pub fn new(
        name: String,
        storage: Vec<(String, BuildType)>,
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
