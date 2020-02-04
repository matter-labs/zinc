use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileMarker {
    pub file: String,
}

impl FileMarker {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

impl InstructionInfo for FileMarker {
    fn to_assembly(&self) -> String {
        format!("marker: file = \"{}\"", self.file)
    }

    fn code() -> InstructionCode {
        InstructionCode::FileMarker
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
        Instruction::FileMarker((*self).clone())
    }
}
