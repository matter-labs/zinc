use crate::gadgets::{Gadget, IntegerType, Primitive, ScalarType, Gadgets};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
pub use num_bigint::BigInt;
use std::mem;

pub struct ToBits;

impl<E: Engine> Gadget<E> for ToBits {
    type Input = Primitive<E>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        match input.scalar_type {
            ScalarType::Integer(IntegerType { signed: true, .. }) => {
                return signed_to_bits(cs, input);
            },
            _ => {}
        }

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

fn signed_to_bits<E, CS>(mut cs: CS, scalar: Primitive<E>) -> Result<Vec<Primitive<E>>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    let length = match scalar.get_type() {
        ScalarType::Integer(IntegerType { length, signed: true }) => length,
        t => return Err(RuntimeError::TypeError {
            expected: "signed type".to_string(),
            actual: t.to_string(),
        })
    };

    let mut gadgets = Gadgets::new(cs.namespace(|| "gadgets"));

    let base_value = BigInt::from(1) << length;
    let base = gadgets.constant_bigint(&base_value, ScalarType::Field)?;
    let complement = gadgets.add(scalar, base)?;

    mem::drop(gadgets);

    let bits = complement
        .as_allocated_num(cs.namespace(|| "num"))?
        .into_bits_le_fixed(cs.namespace(|| "bits"), length + 1)?;

    Ok(bits[..length]
        .iter()
        .rev()
        .map(|b| {
            Primitive::new(
                b.get_value_field::<E>(),
                b.get_variable().expect("must allocate").get_variable(),
                ScalarType::Boolean,
            )
        })
        .collect()
    )
}
