use crate::{Bytecode, RuntimeError};
use num_bigint::BigInt;

pub fn decode_constant(len: u8, bytecode: &mut Bytecode) -> Result<BigInt, RuntimeError> {
    let bytes = bytecode.next_bytes(len as usize).ok_or(RuntimeError::InvalidArguments)?;

    let mut constant = BigInt::from(0);

    for (i, &b) in bytes.iter().enumerate() {
        constant += (b as usize) << (8 * i);
    }

    Ok(constant)
}
