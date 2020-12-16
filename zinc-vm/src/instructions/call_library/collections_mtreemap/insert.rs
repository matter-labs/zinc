//!
//! The `std::collections::MTreeMap::insert` function call.
//!

use std::collections::HashMap;

use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Insert {
    input_size: usize,
    output_size: usize,
}

impl Insert {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            input_size,
            output_size,
        }
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Insert {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let mut storages = storages.ok_or(Error::OnlyForContracts)?;

        let mut input = Vec::with_capacity(self.input_size);
        for _ in 0..self.input_size {
            input.push(state.evaluation_stack.pop()?.try_into_value()?);
        }

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

        let (mut data, key_size, value_size) = match storages
            .get(&eth_address)
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            .load(index.clone())?
            .leaf_values
        {
            LeafVariant::Map {
                data,
                key_size,
                value_size,
            } => (data, key_size, value_size),
            LeafVariant::Array(_array) => return Err(Error::InvalidStorageValue),
        };

        let mut key: Vec<Scalar<E>> = input.drain(value_size..).collect();
        key.reverse();
        let mut value = input;
        value.reverse();

        let position = data
            .iter()
            .position(|(map_key, _value)| map_key.as_slice() == key.as_slice());
        let output = match position {
            Some(position) => data[position].1.to_owned(),
            None => vec![Scalar::new_constant_bool(false); self.output_size - 1],
        };

        for value in output.into_iter() {
            state.evaluation_stack.push(Cell::Value(value))?;
        }
        state
            .evaluation_stack
            .push(Cell::Value(Scalar::new_constant_bool(position.is_some())))?;

        match position {
            Some(position) => data[position].1 = value,
            None => data.push((key, value)),
        }

        if state
            .conditions_stack
            .iter()
            .map(|value| value.get_value().expect(zinc_const::panic::DATA_CONVERSION))
            .all(|value| !value.is_zero())
        {
            storages
                .get_mut(&eth_address)
                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                .store(
                    index,
                    LeafVariant::Map {
                        data,
                        key_size,
                        value_size,
                    },
                )?;
        }

        Ok(())
    }
}
