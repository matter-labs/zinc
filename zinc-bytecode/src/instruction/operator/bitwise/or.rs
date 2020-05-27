//!
//! The 'bitwise OR' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitwiseOr;

impl InstructionInfo for BitwiseOr {
    fn to_assembly(&self) -> String {
        "bit_or".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::BitwiseOr(self)
    }
}
