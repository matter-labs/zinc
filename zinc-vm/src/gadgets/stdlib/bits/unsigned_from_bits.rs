use crate::gadgets::{Gadget, Primitive, ScalarType, IntegerType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;

pub struct UnsignedFromBits;

impl<E: Engine> Gadget<E> for UnsignedFromBits {
    type Input = Vec<Primitive<E>>;
    type Output = Primitive<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let (data_type, length) = if input.len() == (E::Fr::NUM_BITS as usize) {
            (ScalarType::Field, E::Fr::NUM_BITS as usize)
        } else {
            let data_type = ScalarType::Integer(IntegerType {
                signed: false,
                length: input.len(),
            });
            (data_type, input.len())
        };

        let mut bits = Vec::with_capacity(length);
        for (i, value) in input.iter().enumerate() {
            let bit = value.value.map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }
        bits.reverse();

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        Ok(Primitive {
            value: num.get_value(),
            variable: num.get_variable(),
            scalar_type: data_type,
        })
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        vec![output]
    }
}
