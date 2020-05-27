//!
//! The 'logical XOR' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Xor;

impl InstructionInfo for Xor {
    fn to_assembly(&self) -> String {
        "xor".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Xor(self)
    }
}
