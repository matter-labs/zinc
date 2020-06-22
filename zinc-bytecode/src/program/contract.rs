//!
//! The Zinc VM bytecode contract program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;
use crate::program::unit_test::UnitTest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub input: DataType,
    pub output: DataType,
    pub instructions: Vec<Instruction>,
    pub storage: Vec<(String, DataType)>,
    pub unit_test: Option<UnitTest>,
}

impl Contract {
    pub fn new(
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
        unit_test: Option<UnitTest>,
    ) -> Self {
        Self {
            input,
            output,
            instructions,
            storage,
            unit_test,
        }
    }
}
