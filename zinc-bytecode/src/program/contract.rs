//!
//! The Zinc VM bytecode contract program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

///
/// The contract program.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// The circuit name.
    pub name: String,
    /// The contract entry input arguments structure type.
    pub input: DataType,
    /// The contract entry output type.
    pub output: DataType,
    /// The contract bytecode instructions.
    pub instructions: Vec<Instruction>,
    /// The contract storage structure.
    pub storage: Vec<(String, DataType)>,
    /// The unit test data which is present if the contract is a unit test.
    pub unit_test: Option<UnitTest>,
}

impl Contract {
    ///
    /// Creates a contract program instance.
    ///
    pub fn new(
        name: String,
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
        unit_test: Option<UnitTest>,
    ) -> Self {
        Self {
            name,
            input,
            output,
            instructions,
            storage,
            unit_test,
        }
    }
}
