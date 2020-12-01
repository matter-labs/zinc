//!
//! The virtual machine core facade.
//!

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;

use crate::error::VerificationError;
use crate::gadgets;
use crate::IEngine;

pub struct Facade;

impl Facade {
    pub fn verify<E: IEngine>(
        verifying_key: VerifyingKey<E>,
        proof: Proof<E>,
        public_input: zinc_types::Value,
    ) -> Result<bool, VerificationError> {
        let public_input_flat = public_input
            .into_flat_values()
            .into_iter()
            .map(|value| {
                gadgets::scalar::fr_bigint::bigint_to_fr::<E>(&value)
                    .ok_or(VerificationError::ValueOverflow(value))
            })
            .collect::<Result<Vec<E::Fr>, VerificationError>>()?;

        let prepared_verifying_key = groth16::prepare_verifying_key(&verifying_key);
        let success = groth16::verify_proof(
            &prepared_verifying_key,
            &proof,
            public_input_flat.as_slice(),
        )
        .map_err(VerificationError::SynthesisError)?;

        Ok(success)
    }
}
