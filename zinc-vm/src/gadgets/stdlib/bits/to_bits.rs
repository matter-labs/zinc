use crate::gadgets::{Gadget, Primitive, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::PrimeField;

pub struct ToBits;

impl<E: Engine> Gadget<E> for ToBits {
    type Input = Primitive<E>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let num = input.as_allocated_num(cs.namespace(|| "as_allocated_num"))?;

        let len = match input.data_type {
            Some(t) => t.length,
            None => E::Fr::NUM_BITS as usize,
        };

        let bits = num.into_bits_le_fixed(cs.namespace(|| "into_bits_le"), len)?;

        let scalars = bits
            .into_iter()
            .map(|bit| Primitive {
                value: bit.get_value_field::<E>(),
                variable: bit
                    .get_variable()
                    .expect("into_bits_le_fixed must allocate")
                    .get_variable(),
                data_type: Some(ScalarType::BOOLEAN),
            })
            .collect();

        Ok(scalars)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        assert_eq!(input.len(), 1);
        Ok(input[0].clone())
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
