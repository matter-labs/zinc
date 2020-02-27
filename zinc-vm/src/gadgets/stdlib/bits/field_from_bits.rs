use crate::errors::MalformedBytecode;
use crate::gadgets::{Gadget, Scalar, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;

pub struct FieldFromBits;

impl<E: Engine> Gadget<E> for FieldFromBits {
    type Input = Vec<Scalar<E>>;
    type Output = Scalar<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        if input.len() != E::Fr::NUM_BITS as usize {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "FieldFromBits expects exactly {} arguments",
                E::Fr::NUM_BITS
            ))
            .into());
        }

        let mut bits = Vec::with_capacity(E::Fr::NUM_BITS as usize);
        for (i, value) in input.iter().rev().enumerate() {
            let bit = value.get_value().map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        Ok(Scalar::new_unchecked_variable(
            num.get_value(),
            num.get_variable(),
            ScalarType::Field,
        ))
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        vec![output]
    }
}
