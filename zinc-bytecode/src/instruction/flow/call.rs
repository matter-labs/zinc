//!
//! The 'function call' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub address: usize,
    pub inputs_count: usize,
}

impl Call {
    pub fn new(address: usize, inputs_count: usize) -> Self {
        Self {
            address,
            inputs_count,
        }
    }
}

impl InstructionInfo for Call {
    fn to_assembly(&self) -> String {
        format!("call {} {}", self.address, self.inputs_count)
    }

    fn wrap(self) -> Instruction {
        Instruction::Call(self)
    }
}
