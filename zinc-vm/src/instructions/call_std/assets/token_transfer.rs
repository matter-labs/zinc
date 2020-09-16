//!
//! The `std::assets::Token::transfer` function call.
//!

use num_bigint::ToBigInt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::contract::output::transfer::Transfer as TransferOutput;
use crate::core::execution_state::ExecutionState;
use crate::error::RuntimeError;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Transfer;

impl<E: IEngine> INativeCallable<E> for Transfer {
    fn call<CS>(&self, _cs: CS, state: &mut ExecutionState<E>) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let amount = state.evaluation_stack.pop()?.try_into_value()?;
        let token_id = state.evaluation_stack.pop()?.try_into_value()?;
        let to = state.evaluation_stack.pop()?.try_into_value()?;
        let from = state.evaluation_stack.pop()?.try_into_value()?;

        let token_id = token_id
            .to_bigint()
            .expect(zinc_const::panic::DATA_VALID)
            .to_biguint()
            .expect(zinc_const::panic::DATA_VALID);

        let (_sign, from) = from
            .to_bigint()
            .expect(zinc_const::panic::DATA_VALID)
            .to_bytes_be();
        let mut from_array = [0; zinc_const::size::ETH_ADDRESS];
        for (index, byte) in from.into_iter().enumerate() {
            from_array[index] = byte;
        }

        let (_sign, to) = to
            .to_bigint()
            .expect(zinc_const::panic::DATA_VALID)
            .to_bytes_be();
        let mut to_array = [0; zinc_const::size::ETH_ADDRESS];
        for (index, byte) in to.into_iter().enumerate() {
            to_array[index] = byte;
        }

        let amount = amount
            .to_bigint()
            .expect(zinc_const::panic::DATA_VALID)
            .to_biguint()
            .expect(zinc_const::panic::DATA_VALID);

        state
            .transfers
            .push(TransferOutput::new(token_id, from_array, to_array, amount));

        Ok(())
    }
}
