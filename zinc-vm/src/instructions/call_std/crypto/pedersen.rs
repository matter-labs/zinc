//!
//! The `std::crypto::pedersen` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::pedersen_hash;
use franklin_crypto::circuit::pedersen_hash::Personalization;

use crate::core::execution_state::ExecutionState;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Pedersen {
    message_length: usize,
}

impl Pedersen {
    pub fn new(message_length: usize) -> Result<Self, RuntimeError> {
        Ok(Self { message_length })
    }
}

impl<E: IEngine> INativeCallable<E> for Pedersen {
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

        let digest = pedersen_hash::pedersen_hash(
            cs,
            Personalization::NoteCommitment,
            bits.as_slice(),
            E::jubjub_params(),
        )?;

        state
            .evaluation_stack
            .push(Scalar::from(digest.get_x()).into())?;
        state
            .evaluation_stack
            .push(Scalar::from(digest.get_y()).into())?;

        Ok(())
    }
}