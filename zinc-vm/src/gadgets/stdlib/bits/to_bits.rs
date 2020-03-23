use crate::gadgets;
use crate::gadgets::{Gadget, IntegerType, Scalar, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
pub use num_bigint::BigInt;

pub struct ToBits;

impl<E: Engine> Gadget<E> for ToBits {
    type Input = Scalar<E>;
    type Output = Vec<Scalar<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        if let ScalarType::Integer(IntegerType { signed: true, .. }) = input.get_type() {
            return signed_to_bits(cs, input);
        }

        let num = input.to_expression::<CS>();

        let mut bits = match input.get_type() {
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
            .map(|bit| {
                Scalar::new_unchecked_variable(
                    bit.get_value_field::<E>(),
                    bit.get_variable()
                        .expect("into_bits_le_fixed must allocate")
                        .get_variable(),
                    ScalarType::Boolean,
                )
            })
            .collect();

        Ok(scalars)
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        assert_eq!(input.len(), 1);
        Ok(input[0].clone())
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        output
    }
}

fn signed_to_bits<E, CS>(mut cs: CS, scalar: Scalar<E>) -> Result<Vec<Scalar<E>>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let length = match scalar.get_type() {
        ScalarType::Integer(IntegerType {
            length,
            signed: true,
        }) => length,
        t => {
            return Err(RuntimeError::TypeError {
                expected: "signed type".to_string(),
                actual: t.to_string(),
            })
        }
    };

    let base_value = BigInt::from(1) << length;
    let base = Scalar::new_constant_bigint(&base_value, ScalarType::Field)?;

    let complement = gadgets::add(cs.namespace(|| "complement"), &scalar, &base)?;


    let bits = complement
        .to_expression::<CS>()
        .into_bits_le_fixed(cs.namespace(|| "bits"), length + 1)?;

    Ok(bits[..length]
        .iter()
        .rev()
        .map(|b| {
            Scalar::new_unchecked_variable(
                b.get_value_field::<E>(),
                b.get_variable().expect("must allocate").get_variable(),
                ScalarType::Boolean,
            )
        })
        .collect())
}
