//!
//! The 'lesser or equal comparison' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Le;

impl InstructionInfo for Le {
    fn to_assembly(&self) -> String {
        "le".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Le(self)
    }
}
