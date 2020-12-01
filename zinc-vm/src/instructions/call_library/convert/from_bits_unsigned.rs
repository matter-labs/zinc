//!
//! The `std::convert::from_bits_unsigned` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct FromBitsUnsigned {
    bitlength: usize,
}

impl FromBitsUnsigned {
    pub fn new(inputs_count: usize) -> Self {
        Self {
            bitlength: inputs_count,
        }
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for FromBitsUnsigned {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error> {
        if self.bitlength > E::Fr::CAPACITY as usize {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "unsigned_from_bits: integer type with length {} is not supported",
                self.bitlength
            ))
            .into());
        }

        let mut bits = Vec::with_capacity(self.bitlength);
        for i in 0..self.bitlength {
            let bit = state.evaluation_stack.pop()?.try_into_value()?;
            let boolean = bit.to_boolean(cs.namespace(|| format!("to_boolean {}", i)))?;
            bits.push(boolean);
        }

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        let int_type = zinc_types::IntegerType {
            is_signed: false,
            bitlength: self.bitlength,
        };

        let scalar =
            Scalar::new_unchecked_variable(num.get_value(), num.get_variable(), int_type.into());

        state.evaluation_stack.push(scalar.into())?;

        Ok(())
    }
}
