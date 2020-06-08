//!
//! The virtual machine contract synthesizer.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use franklin_crypto::bellman;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;

use zinc_bytecode::Program as BytecodeProgram;

use crate::constraint_systems::dedup::DedupCS;
use crate::constraint_systems::logging::LoggingCS;
use crate::core::contract::Contract;
use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

pub struct Synthesizer<'a, E: IEngine, S: IMerkleTree<E>> {
    pub inputs: Option<Vec<BigInt>>,
    pub output: &'a mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
    pub bytecode: BytecodeProgram,
    pub storage: S,

    pub _pd: PhantomData<E>,
}

impl<E, S> bellman::Circuit<E> for Synthesizer<'_, E, S>
where
    E: IEngine,
    S: IMerkleTree<E>,
{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let storage = StorageGadget::<_, _, Sha256Hasher>::new(
            cs.namespace(|| "storage init"),
            self.storage,
        )?;

        let mut contract = Contract::new(DedupCS::new(LoggingCS::new(cs)), storage, false);
        *self.output =
            Some(contract.run(&self.bytecode, self.inputs.as_deref(), |_| {}, |_| Ok(())));

        Ok(())
    }
}
