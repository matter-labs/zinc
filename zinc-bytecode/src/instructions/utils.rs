use crate::{vlq, DecodingError, InstructionCode, InstructionInfo};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

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

pub fn encode_with_usize(code: InstructionCode, values: &[usize]) -> Vec<u8> {
    let mut bytes = vec![code as u8];
    for &value in values.iter() {
        bytes.append(vlq::encode(&BigInt::from(value)).as_mut());
    }
    bytes
}

pub fn decode_with_usize(
    code: InstructionCode,
    bytes: &[u8],
    args_count: usize,
) -> Result<(Vec<usize>, usize), DecodingError> {
    if bytes.is_empty() {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != code as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        let mut args = Vec::new();
        let mut len = 1;
        for _ in 0..args_count {
            let (bigint, arg_len) =
                vlq::decode(&bytes[len..]).ok_or(DecodingError::UnexpectedEOF)?;
            let value = bigint.to_usize().ok_or(DecodingError::ConstantTooLong)?;
            args.push(value);
            len += arg_len;
        }
        Ok((args, len))
    }
}

pub fn decode_with_two_usize<T>(bytes: &[u8]) -> Result<(T, usize), DecodingError>
where
    T: InstructionInfo + From<(usize, usize)>,
{
    if bytes.is_empty() {
        Err(DecodingError::UnexpectedEOF)
    } else if bytes[0] != T::code() as u8 {
        Err(DecodingError::UnknownInstructionCode(bytes[0]))
    } else {
        let (bigint1, len1) = vlq::decode(&bytes[1..]).ok_or(DecodingError::UnexpectedEOF)?;
        let value1 = bigint1.to_usize().ok_or(DecodingError::ConstantTooLong)?;

        let (bigint2, len2) = vlq::decode(&bytes[1..]).ok_or(DecodingError::UnexpectedEOF)?;
        let value2 = bigint2.to_usize().ok_or(DecodingError::ConstantTooLong)?;

        Ok((T::from((value1, value2)), len1 + len2 + 1))
    }
}

pub fn encode_with_bigint(code: InstructionCode, value: &BigInt) -> Vec<u8> {
    let mut bytes = vec![code as u8];
    bytes.append(vlq::encode(value).as_mut());
    bytes
}

pub fn decode_with_bigint(
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
