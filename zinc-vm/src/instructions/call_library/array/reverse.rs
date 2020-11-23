//!
//! The `std::array::reverse` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Reverse {
    array_length: usize,
}

impl Reverse {
    pub fn new(inputs_count: usize) -> Result<Self, Error> {
        Ok(Self {
            array_length: inputs_count,
        })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Reverse {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error> {
        let mut array = Vec::with_capacity(self.array_length);

        for _ in 0..self.array_length {
            let value = state.evaluation_stack.pop()?;
            array.push(value);
        }

        for value in array {
            state.evaluation_stack.push(value)?;
        }

        Ok(())
    }
}
