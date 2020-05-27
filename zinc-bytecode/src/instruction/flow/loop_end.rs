//!
//! The 'loop end' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopEnd;

impl InstructionInfo for LoopEnd {
    fn to_assembly(&self) -> String {
        "loop_end".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::LoopEnd(self)
    }
}
