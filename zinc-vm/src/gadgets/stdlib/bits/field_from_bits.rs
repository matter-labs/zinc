use crate::gadgets::{Gadget, Primitive};
use crate::RuntimeError;
use crate::ZincEngine;
use bellman::ConstraintSystem;
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;

pub struct FieldFromBits;

impl<E: ZincEngine> Gadget<E> for FieldFromBits {
    type Input = Vec<Primitive<E>>;
    type Output = Primitive<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        if input.len() != E::Fr::NUM_BITS as usize {
            return Err(RuntimeError::InvalidArguments(format!(
                "FieldFromBits expects exactly {} arguments",
                E::Fr::NUM_BITS
            )));
        }

        let mut bits = Vec::with_capacity(E::Fr::NUM_BITS as usize);
        for (i, value) in input.iter().enumerate() {
            let bit = value.value.map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        Ok(Primitive {
            value: num.get_value(),
            variable: num.get_variable(),
            data_type: None,
        })
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        vec![output]
    }
}
