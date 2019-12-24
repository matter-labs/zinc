use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub struct LoadByRef;

impl LoadByRef {
    pub fn new() -> Self {
        Self
    }
}

impl InstructionInfo for LoadByRef {
    fn to_assembly(&self) -> String {
        "load_by_ref".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadByRef
    }

    fn encode(&self) -> Vec<u8> {
        vec![Self::code() as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        utils::decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadByRef((*self).clone())
    }
}

impl Default for LoadByRef {
    fn default() -> Self {
        Self::new()
    }
}
