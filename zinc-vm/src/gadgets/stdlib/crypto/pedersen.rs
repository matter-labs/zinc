use crate::gadgets::{Gadget, Primitive};
use crate::{RuntimeError, Engine};
use bellman::ConstraintSystem;

use franklin_crypto::circuit::baby_pedersen_hash::{pedersen_hash, Personalization};

use franklin_crypto::circuit::baby_ecc::EdwardsPoint;

pub struct Pedersen;

impl<E: Engine> Gadget<E> for Pedersen {
    type Input = Vec<Primitive<E>>;
    type Output = EdwardsPoint<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let mut bits = Vec::new();
        for (i, byte) in input.into_iter().enumerate() {
            let byte_num =
                byte.as_allocated_num(cs.namespace(|| format!("as_allocated_num {}", i)))?;
            let mut byte_bits = byte_num
                .into_bits_le_fixed(cs.namespace(|| format!("into_bits_le_fixed {}", i)), 8)?;
            bits.append(&mut byte_bits)
        }

        let digest = pedersen_hash(
            cs,
            Personalization::NoteCommitment,
            bits.as_slice(),
            E::jubjub_params(),
        )?;

        Ok(digest)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        [output.get_x(), output.get_y()]
            .iter()
            .map(|&num| Primitive {
                value: num.get_value(),
                variable: num.get_variable(),
                data_type: None,
            })
            .collect()
    }
}
