//!
//! The 'arithmetic addition' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Add;

impl InstructionInfo for Add {
    fn to_assembly(&self) -> String {
        "add".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Add(self)
    }
}
