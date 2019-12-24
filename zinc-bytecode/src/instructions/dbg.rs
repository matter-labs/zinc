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
        let mut bytes = vec![InstructionCode::Log as u8];
        let mut utf8: Vec<u8> = self.string.bytes().clone().collect();
        bytes.push(self.nargs as u8);
        bytes.push(utf8.len() as u8);
        bytes.append(utf8.as_mut());
        bytes
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        if bytes.len() < 3 {
            return Err(DecodingError::UnexpectedEOF);
        }

        if bytes[0] != Self::code() as u8 {
            return Err(DecodingError::UnknownInstructionCode(bytes[0]));
        }

        let nargs = bytes[1] as usize;
        let string_len = bytes[2] as usize;

        if bytes.len() < 3 + string_len {
            return Err(DecodingError::UnexpectedEOF);
        }

        let string = String::from_utf8(Vec::from(&bytes[3..3+ string_len]))
            .map_err(|_| DecodingError::UTF8Error)?;

        Ok((Self::new(string, nargs), 3 + string_len))
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
