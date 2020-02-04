use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocationMarker {
    pub file: String,
    pub line: usize,
}

impl LocationMarker {
    pub fn new(file: String, line: usize) -> Self {
        Self { file, line }
    }
}

impl InstructionInfo for LocationMarker {
    fn to_assembly(&self) -> String {
        format!("location {}:{}", self.file, self.line)
    }

    fn code() -> InstructionCode {
        InstructionCode::LocationMarker
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
        Instruction::LocationMarker((*self).clone())
    }
}
