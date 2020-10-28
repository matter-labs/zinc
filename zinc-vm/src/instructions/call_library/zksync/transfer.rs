//!
//! The `zksync::transfer` function call.
//!

use num::bigint::ToBigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::output::transfer::Transfer as TransferOutput;
use crate::core::execution_state::ExecutionState;
use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Transfer;

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Transfer {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storage: Option<&mut S>,
    ) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let amount = state.evaluation_stack.pop()?.try_into_value()?;
        let token_address = state.evaluation_stack.pop()?.try_into_value()?;
        let recipient = state.evaluation_stack.pop()?.try_into_value()?;

        let token_address = token_address
            .to_bigint()
            .unwrap_or_default()
            .to_biguint()
            .unwrap_or_default();

        let (_sign, recipient) = recipient.to_bigint().unwrap_or_default().to_bytes_be();
        let mut recipient_array = [0; zinc_const::size::ETH_ADDRESS];
        for (index, byte) in recipient.into_iter().enumerate() {
            recipient_array[index] = byte;
        }

        let amount = amount
            .to_bigint()
            .unwrap_or_default()
            .to_biguint()
            .unwrap_or_default();

        state
            .transfers
            .push(TransferOutput::new(recipient_array, token_address, amount));

        Ok(())
    }
}
