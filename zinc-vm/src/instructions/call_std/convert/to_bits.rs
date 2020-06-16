//!
//! The `std::convert::to_bits` function call.
//!

use num_bigint::BigInt;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;

use zinc_bytecode::IntegerType;
use zinc_bytecode::ScalarType;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct ToBits;

impl<E: IEngine> INativeCallable<E> for ToBits {
    fn call<CS: ConstraintSystem<E>>(
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
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let bitlength = match scalar.get_type() {
        ScalarType::Integer(IntegerType {
            bitlength,
            is_signed: true,
        }) => bitlength,
        r#type => {
            return Err(RuntimeError::TypeError {
                expected: "signed type".to_string(),
                actual: r#type.to_string(),
            })
        }
    };

    let base_value = BigInt::from(1) << bitlength;
    let base = Scalar::new_constant_bigint(base_value, ScalarType::Field)?;

    let complement = gadgets::arithmetic::add::add(cs.namespace(|| "complement"), &scalar, &base)?;

    let bits = complement
        .to_expression::<CS>()
        .into_bits_le_fixed(cs.namespace(|| "bits"), bitlength + 1)?;

    Ok(Vec::from(&bits[..bitlength]))
}
