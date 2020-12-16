//!
//! The `<Contract>::transfer` function call.
//!

use std::collections::HashMap;

use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Transfer;

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Transfer {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let amount = state.evaluation_stack.pop()?.try_into_value()?;
        let token_address = state.evaluation_stack.pop()?.try_into_value()?;
        let recipient = state.evaluation_stack.pop()?.try_into_value()?;
        let sender = state.evaluation_stack.pop()?.try_into_value()?;

        let sender = zinc_types::address_from_slice(
            sender
                .to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_bytes_be()
                .1
                .as_slice(),
        );
        let recipient = zinc_types::address_from_slice(
            recipient
                .to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_bytes_be()
                .1
                .as_slice(),
        );
        let token_address = zinc_types::address_from_slice(
            token_address
                .to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_bytes_be()
                .1
                .as_slice(),
        );
        let amount = zinc_types::num_compat_backward(
            amount
                .to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_biguint()
                .expect(zinc_const::panic::DATA_CONVERSION),
        );

        if state
            .conditions_stack
            .iter()
            .map(|value| value.get_value().expect(zinc_const::panic::DATA_CONVERSION))
            .all(|value| !value.is_zero())
        {
            state.transfers.push(zinc_types::TransactionMsg::new(
                sender,
                recipient,
                token_address,
                amount,
            ));
        }

        Ok(())
    }
}
