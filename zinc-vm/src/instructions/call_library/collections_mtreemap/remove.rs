//!
//! The `std::collections::MTreeMap::remove` function call.
//!

use num::bigint::ToBigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Remove {
    input_size: usize,
    output_size: usize,
}

impl Remove {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            input_size,
            output_size,
        }
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Remove {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        storage: Option<&mut S>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let storage = storage.ok_or(Error::OnlyForContracts)?;

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
            .unwrap_or_default();
        let (mut data, key_size, value_size) = match storage.load(index.clone())?.leaf_values {
            LeafVariant::Map {
                data,
                key_size,
                value_size,
            } => (data, key_size, value_size),
            LeafVariant::Array(_array) => return Err(Error::InvalidStorageValue),
        };
        let (output, position, found) = data
            .iter()
            .enumerate()
            .find_map(|(index, (map_key, value))| {
                let found = map_key.as_slice() == input.as_slice();
                if found {
                    Some((value.to_owned(), index, found))
                } else {
                    None
                }
            })
            .unwrap_or((
                vec![Scalar::new_constant_bool(false); self.output_size],
                0,
                false,
            ));

        for value in output.into_iter() {
            state.evaluation_stack.push(Cell::Value(value))?;
        }
        state
            .evaluation_stack
            .push(Cell::Value(Scalar::new_constant_bool(found)))?;

        if found {
            data.remove(position);
            storage.store(
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
