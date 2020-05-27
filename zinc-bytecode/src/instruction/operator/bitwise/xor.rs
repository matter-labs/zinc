//!
//! The 'bitwise XOR' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseXor;

impl InstructionInfo for BitwiseXor {
    fn to_assembly(&self) -> String {
        "bit_xor".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseXor(self)
    }
}
