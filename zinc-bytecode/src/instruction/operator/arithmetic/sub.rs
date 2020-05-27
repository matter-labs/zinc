//!
//! The 'arithmetic subtraction' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sub;

impl InstructionInfo for Sub {
    fn to_assembly(&self) -> String {
        "sub".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Sub(self)
    }
}
