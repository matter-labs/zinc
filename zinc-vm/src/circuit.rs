//!
//! The Zinc virtual machine facade.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use franklin_crypto::bellman::Circuit;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;

use zinc_bytecode::Program;

use crate::constraint_systems::duplicate_removing::DuplicateRemovingCS;
use crate::core::VMState;
use crate::error::RuntimeError;
use crate::gadgets::contract::storage::StorageGadget;
use crate::gadgets::contract::MerkleTreeStorage;
use crate::gadgets::contract::Sha256Hasher;
use crate::Engine;

pub struct VMCircuit<'a, E: Engine, S: MerkleTreeStorage<E>> {
    pub program: &'a Program,
    pub inputs: Option<&'a [BigInt]>,
    pub result: &'a mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
    pub storage: S,

    pub _pd: PhantomData<E>,
}

impl<E, S> Circuit<E> for VMCircuit<'_, E, S>
where
    E: Engine,
    S: MerkleTreeStorage<E>,
{
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS,
    ) -> std::result::Result<(), SynthesisError> {
        // let cs = LoggingConstraintSystem::new(cs.namespace(|| "logging"));
        let mut cs = DuplicateRemovingCS::new(cs.namespace(|| "duplicates removing"));

        let storage = StorageGadget::<_, _, Sha256Hasher>::new(
            cs.namespace(|| "storage init"),
            self.storage,
        )?;

        let mut vm = VMState::new(cs.namespace(|| "vm"), storage, false);
        *self.result = Some(vm.run(self.program, self.inputs, |_| {}, |_| Ok(())));

        Ok(())
    }
}
