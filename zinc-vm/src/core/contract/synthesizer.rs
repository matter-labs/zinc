//!
//! The virtual machine contract synthesizer.
//!

use std::collections::HashMap;
use std::marker::PhantomData;

use num::BigInt;

use franklin_crypto::bellman;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;

use crate::constraint_systems::dedup::Dedup as DedupCS;
use crate::constraint_systems::logging::Logging as LoggingCS;
use crate::core::contract::storage::keeper::IKeeper;
use crate::core::contract::State;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

pub struct Synthesizer<'a, E: IEngine, S: IMerkleTree<E>> {
    pub inputs: Option<Vec<BigInt>>,
    pub output: &'a mut Option<Result<Vec<Option<BigInt>>, Error>>,
    pub bytecode: zinc_types::Contract,
    pub method: zinc_types::ContractMethod,
    pub storages: HashMap<BigInt, StorageGadget<E, S, Sha256Hasher>>,
    pub keeper: Box<dyn IKeeper>,
    pub transaction: zinc_types::TransactionMsg,

    pub _pd: PhantomData<E>,
}

impl<E, S> bellman::Circuit<E> for Synthesizer<'_, E, S>
where
    E: IEngine,
    S: IMerkleTree<E>,
{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut contract = State::new(
            DedupCS::new(LoggingCS::new(cs)),
            self.storages,
            self.keeper,
            self.transaction,
        );

        *self.output = Some(contract.run(
            self.bytecode,
            self.method.input,
            self.inputs.as_deref(),
            |_| {},
            |_| Ok(()),
            self.method.address,
        ));

        Ok(())
    }
}
