//!
//! The 'no operation' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoOperation;

impl InstructionInfo for NoOperation {
    fn to_assembly(&self) -> String {
        "noop".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::NoOperation(self)
    }
}
