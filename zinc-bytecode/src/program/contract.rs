//!
//! The Zinc VM bytecode contract program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub input: DataType,
    pub output: DataType,
    pub instructions: Vec<Instruction>,
    pub storage: Vec<(String, DataType)>,
}

impl Contract {
    pub fn new(
        input: DataType,
        output: DataType,
        instructions: Vec<Instruction>,
        storage: Vec<(String, DataType)>,
    ) -> Self {
        Self {
            input,
            output,
            instructions,
            storage,
        }
    }
}
