//!
//! The 'conditional if end' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndIf;

impl InstructionInfo for EndIf {
    fn to_assembly(&self) -> String {
        "endif".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::EndIf(self)
    }
}
