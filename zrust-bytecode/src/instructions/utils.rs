use crate::{vlq, DecodingError, InstructionCode, InstructionInfo};
use num_bigint::BigInt;

pub fn decode_simple_instruction<T>(bytes: &[u8]) -> Result<(T, usize), DecodingError>
where
    T: InstructionInfo + Default,
{
    if bytes.is_empty() {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != T::code() as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        Ok((T::default(), 1))
    }
}

pub fn encode_with_vlq_argument(code: InstructionCode, value: &BigInt) -> Vec<u8> {
    let mut bytes = vec![code as u8];
    bytes.append(vlq::encode(value).as_mut());
    bytes
}

pub fn decode_with_vlq_argument(
    code: InstructionCode,
    bytes: &[u8],
) -> Result<(BigInt, usize), DecodingError> {
    if bytes.len() < 2 {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != code as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        let (value, len) = vlq::decode(&bytes[1..]).ok_or(DecodingError::UnexpectedEOF)?;
        Ok((value, len + 1))
    }
}
