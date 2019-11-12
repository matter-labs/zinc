use crate::{InstructionCode, DecodingError, vlq};
use num_bigint::BigInt;

pub fn decode_simple_instruction<T>(bytes: &[u8], code: InstructionCode, instr: T) -> Result<(T, usize), DecodingError> {
    if bytes.len() < 1 {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != code as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        Ok((instr, 1))
    }
}

pub fn encode_with_vlq_argument(code: InstructionCode, value: &BigInt) -> Vec<u8> {
    let mut bytes = vec![code as u8];
    bytes.append(vlq::encode(value.clone()).as_mut());
    bytes
}

pub fn decode_with_vlq_argument(code: InstructionCode, bytes: &[u8]) -> Result<(BigInt, usize), DecodingError> {
    if bytes.len() < 2 {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != code as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        let (value, len) = vlq::decode(&bytes[1..]).ok_or(DecodingError::UnexpectedEOF)?;
        Ok((value, len + 1))
    }
}
