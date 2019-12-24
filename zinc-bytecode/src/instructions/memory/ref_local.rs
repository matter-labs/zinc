use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Ref {
    pub address: usize,
}

impl Ref {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
}

impl InstructionInfo for Ref {
    fn to_assembly(&self) -> String {
        format!("ref {}", self.address)
    }

    fn code() -> InstructionCode {
        InstructionCode::Ref
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_usize(Self::code(), &[self.address])
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize(Self::code(), bytes, 1)?;

        Ok((Self::new(args[0]), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Ref((*self).clone())
    }
}
