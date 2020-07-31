//!
//! The Zinc VM bytecode circuit program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

///
/// The circuit program.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    /// The circuit name.
    pub name: String,
    /// The circuit entry input arguments structure type.
    pub input: DataType,
    /// The circuit entry output type.
    pub output: DataType,
    /// The circuit bytecode instructions.
    pub instructions: Vec<Instruction>,
    /// The unit test data which is present if the circuit is a unit test.
    pub unit_test: Option<UnitTest>,
}

impl Circuit {
    ///
    /// Creates a circuit program instance.
    ///
    pub fn new(
        name: String,
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        unit_test: Option<UnitTest>,
    ) -> Self {
        Self {
            name,
            input,
            output,
            instructions,
            unit_test,
        }
    }
}