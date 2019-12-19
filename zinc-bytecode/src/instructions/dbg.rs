use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Dbg {
    pub string: String,
    pub nargs: usize,
}

impl Dbg {
    pub fn new(string: String, nargs: usize) -> Self {
        Self { string, nargs }
    }
}

impl InstructionInfo for Dbg {
    fn to_assembly(&self) -> String {
        "debug".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::Log
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Log as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        self.nargs
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::Log((*self).clone())
    }
}
