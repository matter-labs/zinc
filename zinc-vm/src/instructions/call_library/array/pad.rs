//!
//! The `std::array::pad` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Pad {
    array_length: usize,
}

impl Pad {
    pub fn new(inputs_count: usize) -> Result<Self, Error> {
        inputs_count
            .checked_sub(2)
            .map(|array_length| Self { array_length })
            .ok_or_else(|| {
                MalformedBytecode::InvalidArguments(
                    "array::pad expects at least 3 arguments".into(),
                )
                .into()
            })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Pad {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error> {
        let filler = state.evaluation_stack.pop()?.try_into_value()?;
        let new_length = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .get_constant_usize()?;

        if new_length < self.array_length {
            return Err(MalformedBytecode::InvalidArguments(
                "array::pad: new length can't be smaller".into(),
            )
            .into());
        }

        for _ in 0..(new_length - self.array_length) {
            state.evaluation_stack.push(filler.clone().into())?;
        }

        Ok(())
    }
}
