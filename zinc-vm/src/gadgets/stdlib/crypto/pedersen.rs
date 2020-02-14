use crate::gadgets::{Gadget, Primitive, ScalarType};
use crate::{Engine, RuntimeError};
use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::ecc::EdwardsPoint;
use franklin_crypto::circuit::pedersen_hash::{pedersen_hash, Personalization};

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
        for (i, bit_scalar) in input.into_iter().enumerate() {
            let allocated_bit = AllocatedBit::alloc(
                cs.namespace(|| format!("AllocatedBit {}", i)),
                bit_scalar.value.map(|fr| !fr.is_zero()),
            )?;

            bits.push(allocated_bit.into());
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
                scalar_type: ScalarType::Field,
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
