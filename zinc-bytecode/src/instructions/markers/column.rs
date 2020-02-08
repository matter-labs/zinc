use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ColumnMarker {
    pub column: usize,
}

impl ColumnMarker {
    pub fn new(column: usize) -> Self {
        Self { column }
    }
}

impl InstructionInfo for ColumnMarker {
    fn to_assembly(&self) -> String {
        format!("marker: column = \"{}\"", self.column)
    }

    fn code() -> InstructionCode {
        InstructionCode::ColumnMarker
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
        Instruction::ColumnMarker((*self).clone())
    }
}
