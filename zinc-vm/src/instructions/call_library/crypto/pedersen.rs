//!
//! The `std::crypto::pedersen` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::pedersen_hash;
use franklin_crypto::circuit::pedersen_hash::Personalization;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct Pedersen {
    message_length: usize,
}

impl Pedersen {
    pub fn new(message_length: usize) -> Result<Self, Error> {
        Ok(Self { message_length })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Pedersen {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error> {
        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = state
                .evaluation_stack
                .pop()?
                .try_into_value()?
                .to_boolean(cs.namespace(|| format!("bit {}", i)))?;

            bits.push(bit);
        }
        bits.reverse();

        let digest = pedersen_hash::pedersen_hash(
            cs,
            Personalization::NoteCommitment,
            bits.as_slice(),
            E::jubjub_params(),
        )?;

        state
            .evaluation_stack
            .push(Scalar::from(digest.get_x()).into())?;
        state
            .evaluation_stack
            .push(Scalar::from(digest.get_y()).into())?;

        Ok(())
    }
}
