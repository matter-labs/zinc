//!
//! The 'arithmetic multiplication' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mul;

impl InstructionInfo for Mul {
    fn to_assembly(&self) -> String {
        "mul".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Mul(self)
    }
}
