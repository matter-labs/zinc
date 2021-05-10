use crate::core::EvaluationStack;
use crate::gadgets::Scalar;
use crate::stdlib::NativeFunction;
use crate::{Engine, Result};
use bellman::ConstraintSystem;
use franklin_crypto::circuit::pedersen_hash::{pedersen_hash, Personalization};

pub struct PedersenMultiInput {
    message_length: usize,
}

impl PedersenMultiInput {
    pub fn new(message_length: usize) -> Result<Self> {
        Ok(Self { message_length })
    }
}

impl<E: Engine> NativeFunction<E> for PedersenMultiInput {
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

        let digest = pedersen_hash(
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
