//!
//! The 'bitwise NOT' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseNot;

impl InstructionInfo for BitwiseNot {
    fn to_assembly(&self) -> String {
        "bit_not".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseNot(self)
    }
}
