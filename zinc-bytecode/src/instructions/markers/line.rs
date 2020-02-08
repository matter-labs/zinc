use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LineMarker {
    pub line: usize,
}

impl LineMarker {
    pub fn new(line: usize) -> Self {
        Self { line }
    }
}

impl InstructionInfo for LineMarker {
    fn to_assembly(&self) -> String {
        format!("marker: line = \"{}\"", self.line)
    }

    fn code() -> InstructionCode {
        InstructionCode::LineMarker
    }

    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn decode(_bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        unimplemented!()
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::LineMarker((*self).clone())
    }
}
