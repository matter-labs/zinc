//!
//! The 'conditional else' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Else;

impl InstructionInfo for Else {
    fn to_assembly(&self) -> String {
        "else".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Else(self)
    }
}
