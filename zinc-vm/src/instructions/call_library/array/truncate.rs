//!
//! The `std::array::truncate` function call.
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

pub struct Truncate {
    array_length: usize,
}

impl Truncate {
    pub fn new(inputs_count: usize) -> Result<Self, Error> {
        inputs_count
            .checked_sub(1)
            .map(|array_length| Self { array_length })
            .ok_or_else(|| {
                MalformedBytecode::InvalidArguments(
                    "array::truncate expects at least 2 arguments".into(),
                )
                .into()
            })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Truncate {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error> {
        let new_length = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .get_constant_usize()?;

        if new_length > self.array_length {
            return Err(MalformedBytecode::InvalidArguments(
                "array::truncate: new length can't be smaller".into(),
            )
            .into());
        }

        for _ in 0..(self.array_length - new_length) {
            state.evaluation_stack.pop()?;
        }

        Ok(())
    }
}
