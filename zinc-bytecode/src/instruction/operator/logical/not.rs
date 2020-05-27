//!
//! The 'logical NOT' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Not;

impl InstructionInfo for Not {
    fn to_assembly(&self) -> String {
        "not".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Not(self)
    }
}
