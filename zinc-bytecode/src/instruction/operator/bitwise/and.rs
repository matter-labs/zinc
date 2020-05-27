//!
//! The 'bitwise AND' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseAnd;

impl InstructionInfo for BitwiseAnd {
    fn to_assembly(&self) -> String {
        "bit_and".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseAnd(self)
    }
}
