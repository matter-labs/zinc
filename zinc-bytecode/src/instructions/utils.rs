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

pub fn encode_with_args<T: Into<BigInt> + Clone>(code: InstructionCode, values: &[T]) -> Vec<u8> {
    let mut bytes = vec![code as u8];
    for value in values {
        let bigint = value.clone().into();
        bytes.append(vlq::encode(&bigint).as_mut());
    }
    bytes
}

/// If Ok is returned, vector is guaranteed to have `args_count` values.
pub fn decode_with_bigint_args(
    code: InstructionCode,
    bytes: &[u8],
    args_count: usize,
) -> Result<(Vec<BigInt>, usize), DecodingError> {
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
            args.push(bigint);
            len += arg_len;
        }
        Ok((args, len))
    }
}

/// If Ok is returned, vector is guaranteed to have `args_count` values.
pub fn decode_with_usize_args(
    code: InstructionCode,
    bytes: &[u8],
    args_count: usize,
) -> Result<(Vec<usize>, usize), DecodingError> {
    let (args, len) = decode_with_bigint_args(code, bytes, args_count)?;
    let mut usize_args = Vec::new();
    for bigint in args {
        let value = bigint.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        usize_args.push(value)
    }

    Ok((usize_args, len))
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
