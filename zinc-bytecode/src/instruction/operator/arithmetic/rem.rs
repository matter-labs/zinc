//!
//! The 'arithmetic remainder' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rem;

impl InstructionInfo for Rem {
    fn to_assembly(&self) -> String {
        "rem".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Rem(self)
    }
}
