//!
//! The `std::crypto::sha256` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::sha256;

use crate::core::execution_state::ExecutionState;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Sha256 {
    message_length: usize,
}

impl Sha256 {
    pub fn new(message_length: usize) -> Result<Self, RuntimeError> {
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

impl<E: IEngine> INativeCallable<E> for Sha256 {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        state: &mut ExecutionState<E>,
    ) -> Result<(), RuntimeError> {
        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = state
                .evaluation_stack
                .pop()?
                .try_into_value()?
                .to_boolean(cs.namespace(|| format!("bit {}", i)))?;

            bits.push(bit);
        }
        bits.reverse();

        let digest_bits = sha256::sha256(cs.namespace(|| "sha256"), &bits)?;

        assert_eq!(digest_bits.len(), 256);

        for bit in digest_bits {
            let scalar = Scalar::from_boolean(cs.namespace(|| "from_boolean"), bit)?;
            state.evaluation_stack.push(scalar.into())?;
        }

        Ok(())
    }
}
