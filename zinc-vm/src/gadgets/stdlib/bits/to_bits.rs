use crate::gadgets::{Gadget, IntegerType, Primitive, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

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

        let mut bits = match input.scalar_type {
            ScalarType::Integer(t) => {
                num.into_bits_le_fixed(cs.namespace(|| "into_bits_le"), t.length)
            }
            ScalarType::Boolean => num.into_bits_le_fixed(cs.namespace(|| "into_bits_le"), 1),
            ScalarType::Field => num.into_bits_le_strict(cs.namespace(|| "into_bits_le_strict")),
        }?;

        // We use big-endian
        bits.reverse();

        let scalars = bits
            .into_iter()
            .map(|bit| Primitive {
                value: bit.get_value_field::<E>(),
                variable: bit
                    .get_variable()
                    .expect("into_bits_le_fixed must allocate")
                    .get_variable(),
                scalar_type: IntegerType::BOOLEAN.into(),
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
