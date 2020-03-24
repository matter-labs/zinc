use crate::{Instruction, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct EndIf;

impl InstructionInfo for EndIf {
    fn to_assembly(&self) -> String {
        "endif".into()
    }

    fn wrap(&self) -> Instruction {
        Instruction::EndIf((*self).clone())
    }
}
