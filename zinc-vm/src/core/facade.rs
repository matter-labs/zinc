//!
//! The Zinc virtual machine facade interface.
//!

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;
use zinc_const::UnitTestExitCode;
use zinc_mongo::Client as MongoClient;

use crate::error::RuntimeError;
use crate::error::VerificationError;
use crate::gadgets;
use crate::IEngine;

pub trait IFacade {
    fn run<E: IEngine>(
        self,
        input: TemplateValue,
        mongo_client: Option<MongoClient>,
    ) -> Result<TemplateValue, RuntimeError>;

    fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError>;

    fn test<E: IEngine>(self) -> Result<UnitTestExitCode, RuntimeError>;

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError>;

    fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError>;

    fn verify<E: IEngine>(
        verifying_key: VerifyingKey<E>,
        proof: Proof<E>,
        public_input: TemplateValue,
    ) -> Result<bool, VerificationError> {
        let public_input_flat = public_input
            .into_flat_values()
            .into_iter()
            .map(|value| {
                gadgets::scalar::fr_bigint::bigint_to_fr::<E>(&value)
                    .ok_or_else(|| VerificationError::ValueOverflow(value))
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

impl IFacade for BytecodeProgram {
    fn run<E: IEngine>(
        self,
        input: TemplateValue,
        mongo_client: Option<MongoClient>,
    ) -> Result<TemplateValue, RuntimeError> {
        match self {
            BytecodeProgram::Circuit(inner) => inner.run::<E>(input, mongo_client),
            BytecodeProgram::Contract(inner) => inner.run::<E>(input, mongo_client),
        }
    }

    fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        match self {
            BytecodeProgram::Circuit(inner) => inner.debug::<E>(input),
            BytecodeProgram::Contract(inner) => inner.debug::<E>(input),
        }
    }

    fn test<E: IEngine>(self) -> Result<UnitTestExitCode, RuntimeError> {
        match self {
            BytecodeProgram::Circuit(inner) => inner.test::<E>(),
            BytecodeProgram::Contract(inner) => inner.test::<E>(),
        }
    }

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
        match self {
            BytecodeProgram::Circuit(inner) => inner.setup(),
            BytecodeProgram::Contract(inner) => inner.setup(),
        }
    }

    fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        match self {
            BytecodeProgram::Circuit(inner) => inner.prove(params, witness),
            BytecodeProgram::Contract(inner) => inner.prove(params, witness),
        }
    }
}
