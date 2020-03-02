use crate::gadgets::{Gadget, Scalar, ScalarType};
use crate::{Engine, RuntimeError};
use bellman::ConstraintSystem;
use franklin_crypto::circuit::ecc::EdwardsPoint;
use franklin_crypto::circuit::pedersen_hash::{pedersen_hash, Personalization};

pub struct Pedersen;

impl<E: Engine> Gadget<E> for Pedersen {
    type Input = Vec<Scalar<E>>;
    type Output = EdwardsPoint<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let mut bits = Vec::new();
        for (i, bit_scalar) in input.into_iter().enumerate() {
            let boolean = bit_scalar.to_boolean(cs.namespace(|| format!("to_boolean {}", i)))?;
            bits.push(boolean);
        }

        let digest = pedersen_hash(
            cs,
            Personalization::NoteCommitment,
            bits.as_slice(),
            E::jubjub_params(),
        )?;

        Ok(digest)
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        [output.get_x(), output.get_y()]
            .iter()
            .map(|&num| {
                Scalar::new_unchecked_variable(
                    num.get_value(),
                    num.get_variable(),
                    ScalarType::Field,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Engine;
    use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use pairing::bn256::Bn256;

    #[test]
    fn pedersen_test() {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let bits: Vec<_> = (0..248)
            .map(|i| {
                let allocated_bit =
                    AllocatedBit::alloc(cs.namespace(|| format!("{}", i)), Some(false))
                        .expect("alloc bit");
                Boolean::from(allocated_bit)
            })
            .collect();

        pedersen_hash(
            cs.namespace(|| "computation of pedersen hash"),
            Personalization::NoteCommitment,
            &bits,
            Bn256::jubjub_params(),
        )
        .expect("hash");
    }
}
