//!
//! The 'bitwise shift right' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseShiftRight;

impl InstructionInfo for BitwiseShiftRight {
    fn to_assembly(&self) -> String {
        "bit_shift_right".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseShiftRight(self)
    }
}
