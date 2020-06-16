//!
//! The Zinc VM bytecode circuit program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as DataType;
use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub input: DataType,
    pub output: DataType,
    pub instructions: Vec<Instruction>,
}

impl Circuit {
    pub fn new(input: DataType, output: DataType, instructions: Vec<Instruction>) -> Self {
        Self {
            input,
            output,
            instructions,
        }
    }
}
