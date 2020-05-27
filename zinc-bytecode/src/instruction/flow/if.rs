//!
//! The 'conditional if' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct If;

impl InstructionInfo for If {
    fn to_assembly(&self) -> String {
        "if".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::If(self)
    }
}
