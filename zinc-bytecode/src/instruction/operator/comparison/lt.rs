//!
//! The 'lesser comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lt;

impl InstructionInfo for Lt {
    fn to_assembly(&self) -> String {
        "lt".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Lt(self)
    }
}
