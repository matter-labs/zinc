//!
//! The 'greater comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gt;

impl InstructionInfo for Gt {
    fn to_assembly(&self) -> String {
        "gt".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Gt(self)
    }
}
