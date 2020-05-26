use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rem;

impl InstructionInfo for Rem {
    fn to_assembly(&self) -> String {
        "rem".into()
    }

    fn wrap(self) -> Instruction {
        Instruction::Rem(self)
    }
}
