//!
//! The `std::convert::from_bits_field` function.
//!

use bellman::ConstraintSystem;
use ff::PrimeField;
use franklin_crypto::circuit::num::AllocatedNum;

use zinc_bytecode::ScalarType;

use crate::core::state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::stdlib::NativeFunction;
use crate::Engine;

pub struct FieldFromBits;

impl<E: Engine> NativeFunction<E> for FieldFromBits {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError> {
        let mut bits = Vec::with_capacity(E::Fr::NUM_BITS as usize);
        for i in 0..E::Fr::NUM_BITS {
            let bit = stack.pop()?.try_into_value()?;
            let boolean = bit.to_boolean(cs.namespace(|| format!("to_boolean {}", i)))?;
            bits.push(boolean);
        }

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        stack.push(
            Scalar::new_unchecked_variable(num.get_value(), num.get_variable(), ScalarType::Field)
                .into(),
        )?;

        Ok(())
    }
}
