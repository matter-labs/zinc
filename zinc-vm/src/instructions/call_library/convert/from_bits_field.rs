//!
//! The `std::convert::from_bits_field` function call.
//!

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

use zinc_build::ScalarType;

use crate::core::execution_state::ExecutionState;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct FromBitsField;

impl<E: IEngine> INativeCallable<E> for FromBitsField {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        state: &mut ExecutionState<E>,
    ) -> Result<(), RuntimeError> {
        let mut bits = Vec::with_capacity(E::Fr::NUM_BITS as usize);
        for i in 0..E::Fr::NUM_BITS {
            let bit = state.evaluation_stack.pop()?.try_into_value()?;
            let boolean = bit.to_boolean(cs.namespace(|| format!("to_boolean {}", i)))?;
            bits.push(boolean);
        }

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        state.evaluation_stack.push(
            Scalar::new_unchecked_variable(num.get_value(), num.get_variable(), ScalarType::Field)
                .into(),
        )?;

        Ok(())
    }
}
