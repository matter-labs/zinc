//!
//! The `std::crypto::schnorr::Signature::verify` function call.
//!

use std::collections::HashMap;

use num::BigInt;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::baby_eddsa::EddsaSignature;
use franklin_crypto::circuit::ecc::EdwardsPoint;
use franklin_crypto::jubjub::FixedGenerators;
use franklin_crypto::jubjub::JubjubParams;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;

pub struct SchnorrSignatureVerify {
    msg_len: usize,
}

impl SchnorrSignatureVerify {
    pub fn new(args_count: usize) -> Result<Self, Error> {
        if args_count < 6 {
            return Err(MalformedBytecode::InvalidArguments(
                "schnorr::verify needs at least 6 arguments".into(),
            )
            .into());
        }

        Ok(Self {
            msg_len: args_count - 5,
        })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for SchnorrSignatureVerify {
    fn call<CS>(
        &self,
        mut cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        if self.msg_len > E::Fs::CAPACITY as usize {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "maximum message length for schnorr signature is {}",
                E::Fs::CAPACITY
            ))
            .into());
        }

        let mut message = Vec::new();
        for _ in 0..self.msg_len {
            let bit = state.evaluation_stack.pop()?.try_into_value()?;
            message.push(bit);
        }

        let pk_y = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_expression::<CS>()
            .into_number(cs.namespace(|| "to_number pk_y"))?;
        let pk_x = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_expression::<CS>()
            .into_number(cs.namespace(|| "to_number pk_x"))?;
        let s = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_expression::<CS>()
            .into_number(cs.namespace(|| "to_number s"))?;
        let r_y = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_expression::<CS>()
            .into_number(cs.namespace(|| "to_number r_y"))?;
        let r_x = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .to_expression::<CS>()
            .into_number(cs.namespace(|| "to_number r_x"))?;

        let r = EdwardsPoint::interpret(cs.namespace(|| "r"), &r_x, &r_y, E::jubjub_params())?;
        let pk = EdwardsPoint::interpret(cs.namespace(|| "pk"), &pk_x, &pk_y, E::jubjub_params())?;

        let signature = EddsaSignature { r, s, pk };

        let is_valid = verify_signature(
            cs.namespace(|| "verify_signature"),
            &message,
            &signature,
            E::jubjub_params(),
        )?;

        state.evaluation_stack.push(is_valid.into())
    }
}

pub fn verify_signature<E, CS>(
    mut cs: CS,
    message: &[Scalar<E>],
    signature: &EddsaSignature<E>,
    params: &E::Params,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let message_bits = message
        .iter()
        .enumerate()
        .map(|(i, bit)| bit.to_boolean(cs.namespace(|| format!("message bit {}", i))))
        .collect::<Result<Vec<_>, Error>>()?;

    let public_generator = params
        .generator(FixedGenerators::SpendingKeyGenerator)
        .clone();

    let generator = EdwardsPoint::witness(
        cs.namespace(|| "allocate public generator"),
        Some(public_generator),
        params,
    )?;

    let is_verified = signature.is_verified_raw_message_signature(
        cs.namespace(|| "is_verified_signature"),
        params,
        &message_bits,
        generator,
        E::Fr::CAPACITY as usize / 8,
    )?;

    Scalar::from_boolean(cs.namespace(|| "from_boolean"), is_verified)
}
