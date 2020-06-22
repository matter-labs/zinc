//!
//! The Zinc VM bytecode circuit program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub input: DataType,
    pub output: DataType,
    pub instructions: Vec<Instruction>,
    pub unit_test: Option<UnitTest>,
}

impl Circuit {
    pub fn new(
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        unit_test: Option<UnitTest>,
    ) -> Self {
        Self {
            input,
            output,
            instructions,
            unit_test,
        }
    }
}
