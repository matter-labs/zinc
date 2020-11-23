//!
//! The `std::ff::invert` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Inverse;

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Inverse {
    fn call<CS>(
        &self,
        cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let scalar = state.evaluation_stack.pop()?.try_into_value()?;
        let inverse = gadgets::arithmetic::field::inverse(cs, &scalar)?;
        state.evaluation_stack.push(inverse.into())
    }
}
