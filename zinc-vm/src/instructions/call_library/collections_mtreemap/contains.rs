//!
//! The `std::collections::MTreeMap::contains` function call.
//!

use std::collections::HashMap;

use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Contains {
    input_size: usize,
}

impl Contains {
    pub fn new(input_size: usize) -> Self {
        Self { input_size }
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Contains {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let storages = storages.ok_or(Error::OnlyForContracts)?;

        let mut input = Vec::with_capacity(self.input_size);
        for _ in 0..self.input_size {
            input.push(state.evaluation_stack.pop()?.try_into_value()?);
        }
        input.reverse();

        let index = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_bigint()
            .expect(zinc_const::panic::DATA_CONVERSION);
        let eth_address = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_bigint()
            .expect(zinc_const::panic::DATA_CONVERSION);

        let data = match storages
            .get(&eth_address)
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            .load(index)?
            .leaf_values
        {
            LeafVariant::Map { data, .. } => data,
            LeafVariant::Array(_array) => return Err(Error::InvalidStorageValue),
        };
        let found = data.into_iter().any(|(map_key, _value)| map_key == input);

        state
            .evaluation_stack
            .push(Cell::Value(Scalar::new_constant_bool(found)))?;

        Ok(())
    }
}
