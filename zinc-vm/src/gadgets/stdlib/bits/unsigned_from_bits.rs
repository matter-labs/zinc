use crate::gadgets::{Gadget, IntegerType, Scalar, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;

pub struct UnsignedFromBits;

impl<E: Engine> Gadget<E> for UnsignedFromBits {
    type Input = Vec<Scalar<E>>;
    type Output = Scalar<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let (scalar_type, length) = if input.len() == (E::Fr::NUM_BITS as usize) {
            (ScalarType::Field, E::Fr::NUM_BITS as usize)
        } else {
            let data_type = ScalarType::Integer(IntegerType {
                is_signed: false,
                bitlength: input.len(),
            });
            (data_type, input.len())
        };

        let mut bits = Vec::with_capacity(length);
        for (i, value) in input.iter().enumerate() {
            let bit = value.get_value().map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }
        bits.reverse();

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        Ok(Scalar::new_unchecked_variable(
            num.get_value(),
            num.get_variable(),
            scalar_type,
        ))
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        vec![output]
    }
}
