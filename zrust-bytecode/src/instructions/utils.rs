use crate::{InstructionCode, DecodingError};

pub fn decode_simple_instruction<T>(bytes: &[u8], code: InstructionCode, instr: T) -> Result<(T, usize), DecodingError> {
    if bytes.len() < 1 {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != code as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        Ok((instr, 1))
    }
}
