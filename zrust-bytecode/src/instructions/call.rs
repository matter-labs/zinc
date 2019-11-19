use crate::{Instruction, InstructionCode, DecodingError, vlq};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct Call {
    pub address: usize,
    pub inputs_count: usize,
}

impl Instruction for Call {
    fn to_assembly(&self) -> String {
        "call".into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Call
    }

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![InstructionCode::Call as u8];
        bytes.append(vlq::encode(&BigInt::from(self.address)).as_mut());
        bytes.append(vlq::encode(&BigInt::from(self.inputs_count)).as_mut());
        bytes
    }
}

impl Call {
    pub fn new(address: usize, inputs_count: usize) -> Self {
        Self { address, inputs_count }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Call, usize), DecodingError> {
        if bytes.len() < 3 {
            Err(DecodingError::UnexpectedEOF)
        } else if bytes[0] != InstructionCode::Call as u8 {
            Err(DecodingError::UnknownInstructionCode(bytes[0]))
        } else {
            let (address_bi, len1) = vlq::decode(&bytes[1..])
                .ok_or(DecodingError::UnexpectedEOF)?;

            let address = address_bi.to_usize()
                .ok_or(DecodingError::ConstantTooLong)?;

            let (inputs_size_bi, len2) = vlq::decode(&bytes[(len1 + 1)..])
                .ok_or(DecodingError::UnexpectedEOF)?;

            let inputs_size = inputs_size_bi.to_usize()
                .ok_or(DecodingError::ConstantTooLong)?;

            Ok((Self::new(address, inputs_size), 1 + len1 + len2))
        }
    }
}
