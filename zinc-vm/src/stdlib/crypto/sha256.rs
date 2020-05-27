use bellman::ConstraintSystem;
use franklin_crypto::circuit::sha256::sha256;

use crate::core::EvaluationStack;
use crate::error::MalformedBytecode;
use crate::error::Result;
use crate::gadgets::Scalar;
use crate::stdlib::NativeFunction;
use crate::Engine;

pub struct Sha256 {
    message_length: usize,
}

impl Sha256 {
    pub fn new(message_length: usize) -> Result<Self> {
        if message_length % 8 == 0 {
            Ok(Self { message_length })
        } else {
            Err(MalformedBytecode::InvalidArguments(format!(
                "message length for sha256 must be a multiple of 8, got {}",
                message_length
            ))
            .into())
        }
    }
}

impl<E: Engine> NativeFunction<E> for Sha256 {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result {
        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = stack
                .pop()?
                .value()?
                .to_boolean(cs.namespace(|| format!("bit {}", i)))?;

            bits.push(bit);
        }
        bits.reverse();

        let digest_bits = sha256(cs.namespace(|| "sha256"), &bits)?;

        assert_eq!(digest_bits.len(), 256);

        for bit in digest_bits {
            let scalar = Scalar::from_boolean(cs.namespace(|| "from_boolean"), bit)?;
            stack.push(scalar.into())?;
        }

        Ok(())
    }
}
