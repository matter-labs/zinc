//!
//! The 'bitwise shift left' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftLeft;

impl InstructionInfo for BitwiseShiftLeft {
    fn to_assembly(&self) -> String {
        "bit_shift_left".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseShiftLeft(self)
    }
}
