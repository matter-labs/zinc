//!
//! The `std::crypto::pedersen` function.
//!

use bellman::ConstraintSystem;
use franklin_crypto::circuit::pedersen_hash;
use franklin_crypto::circuit::pedersen_hash::Personalization;

use crate::core::state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::stdlib::NativeFunction;
use crate::Engine;

pub struct Pedersen {
    message_length: usize,
}

impl Pedersen {
    pub fn new(message_length: usize) -> Result<Self, RuntimeError> {
        Ok(Self { message_length })
    }
}

impl<E: Engine> NativeFunction<E> for Pedersen {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError> {
        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = stack
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

        stack.push(Scalar::from(digest.get_x()).into())?;
        stack.push(Scalar::from(digest.get_y()).into())?;

        Ok(())
    }
}
