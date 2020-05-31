//!
//! The `std::convert::to_bits` function.
//!

use num_bigint::BigInt;

use bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;

use zinc_bytecode::IntegerType;
use zinc_bytecode::ScalarType;

use crate::core::state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::stdlib::NativeFunction;
use crate::Engine;

pub struct ToBits;

impl<E: Engine> NativeFunction<E> for ToBits {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError> {
        let scalar = stack.pop()?.try_into_value()?;
        let expr = scalar.to_expression::<CS>();

        let mut bits = match scalar.get_type() {
            ScalarType::Boolean => vec![scalar.to_boolean(cs.namespace(|| "to_boolean"))?],
            ScalarType::Integer(t) => {
                if t.is_signed {
                    signed_to_bits(cs.namespace(|| "signed_to_bits"), scalar)?
                } else {
                    expr.into_bits_le_fixed(cs.namespace(|| "into_bits_le"), t.bitlength)?
                }
            }
            ScalarType::Field => {
                expr.into_bits_le_strict(cs.namespace(|| "into_bits_le_strict"))?
            }
        };

        // We use big-endian
        bits.reverse();

        for bit in bits {
            let scalar = Scalar::new_unchecked_variable(
                bit.get_value_field::<E>(),
                bit.get_variable()
                    .expect("into_bits_le_fixed must allocate")
                    .get_variable(),
                ScalarType::Boolean,
            );
            stack.push(scalar.into())?;
        }

        Ok(())
    }
}

fn signed_to_bits<E, CS>(mut cs: CS, scalar: Scalar<E>) -> Result<Vec<Boolean>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let bitlength = match scalar.get_type() {
        ScalarType::Integer(IntegerType {
            bitlength,
            is_signed: true,
        }) => bitlength,
        t => {
            return Err(RuntimeError::TypeError {
                expected: "signed type".to_string(),
                actual: t.to_string(),
            })
        }
    };

    let base_value = BigInt::from(1) << bitlength;
    let base = Scalar::new_constant_bigint(&base_value, ScalarType::Field)?;

    let complement = gadgets::arithmetic::add::add(cs.namespace(|| "complement"), &scalar, &base)?;

    let bits = complement
        .to_expression::<CS>()
        .into_bits_le_fixed(cs.namespace(|| "bits"), bitlength + 1)?;

    Ok(Vec::from(&bits[..bitlength]))
}
