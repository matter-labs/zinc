use crate::{Instruction, InstructionCode, DecodingError, utils, vlq};
use num_traits::ToPrimitive;
use num_bigint::{ToBigInt, BigInt};

#[derive(Debug)]
pub struct LoopBegin {
    pub iterations: usize,
    pub io_size: usize,
}

impl Instruction for LoopBegin {
    fn to_assembly(&self) -> String {
        format!("loop_begin {}", self.iterations).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::LoopBegin
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![InstructionCode::LoopBegin as u8];
        bytes.append(vlq::encode(&BigInt::from(self.iterations)).as_mut());
        bytes.append(vlq::encode(&BigInt::from(self.io_size)).as_mut());
        bytes
    }
}

impl LoopBegin {
    pub fn new(iterations: usize, io_size: usize) -> Self {
        Self { iterations, io_size }
    }

    pub fn decode(bytes: &[u8]) -> Result<(LoopBegin, usize), DecodingError> {
        if bytes.len() < 3 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::LoopBegin as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            let (iterations_bi, iter_len) = vlq::decode(&bytes[1..])
                .ok_or(DecodingError::UnexpectedEOF)?;

            let iterations = iterations_bi.to_usize()
                .ok_or(DecodingError::ConstantTooLong)?;

            let (io_size_bi, io_size_len) = vlq::decode(&bytes[(iter_len + 1)..])
                .ok_or(DecodingError::UnexpectedEOF)?;

            let io_size = io_size_bi.to_usize()
                .ok_or(DecodingError::ConstantTooLong)?;

            Ok((Self::new(iterations, io_size), 1 + iter_len + io_size_len))
        }
    }
}
