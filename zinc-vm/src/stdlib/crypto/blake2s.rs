use crate::core::EvaluationStack;
use crate::gadgets::Scalar;
use crate::stdlib::NativeFunction;
use crate::{Engine, MalformedBytecode, Result};
use bellman::ConstraintSystem;
use franklin_crypto::circuit::blake2s::blake2s;

const BYTE_LENGTH: usize = 8;

pub struct Blake2s {
    message_length: usize,
}

impl Blake2s {
    pub fn new(message_length: usize) -> Result<Self> {
        if message_length % 8 == 0 {
            Ok(Self { message_length })
        } else {
            Err(MalformedBytecode::InvalidArguments(format!(
                "message length for blake2s must be a multiple of 8, got {}",
                message_length
            ))
            .into())
        }
    }
}

impl<E: Engine> NativeFunction<E> for Blake2s {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result {
        //reverse the bits of each byte of the input
        //for compatibility with Bouncy Castle
        fn reverse_byte_bits<T>(bits: &mut Vec<T>) {
            for bytes in bits.chunks_mut(BYTE_LENGTH) {
                bytes.reverse();
            }
        }

        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = stack
                .pop()?
                .value()?
                .to_boolean(cs.namespace(|| format!("bit {}", i)))?;

            bits.push(bit);
        }
        bits.reverse();

        //reverse preimage
        reverse_byte_bits(&mut bits);

        let mut digest_bits = blake2s(cs.namespace(|| "blake2s"), &bits, b"12345678")?;

        //reverse digest
        reverse_byte_bits(&mut digest_bits);

        assert_eq!(digest_bits.len(), 256);

        for bit in digest_bits {
            let scalar = Scalar::from_boolean(cs.namespace(|| "from_boolean"), bit)?;
            stack.push(scalar.into())?;
        }

        Ok(())
    }
}
