use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
use crate::gadgets::{IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use crate::stdlib::bits::signed_to_bits;
use crate::core::RuntimeError;

pub struct StorageGadget<E: Engine> {
    storage: &MerkleTreeStorage<E>,
    root_hash: Scalar<E>,
}

impl<E: Engine> StorageGadget<E> {
    pub fn new<CS>(
        mut cs: CS,
        storage: MerkleTreeStorage<E>,
    ) -> Result<Self>
    where
        CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage.root_hash();
        let root_hash = cs
            .alloc_input(|| "root hash variable", || root_hash_value)
            .map_err(RuntimeError::SynthesisError)?;
        Ok(StorageGadget {
            storage,
            root_hash
        })
    }

    pub fn load<CS>(
        &self,
        mut cs: CS,
        index: &Scalar<E>,
    ) -> Result<Scalar<E>>
    where
        CS: ConstraintSystem<E>,
    {

    }

    pub fn store<CS>(
        &mut self,
        mut cs: CS,
        index: &Scalar<E>,
        value: &Scalar<E>,
    ) -> Result<()>
    where
        CS: ConstraintSystem<E>,
    {

    }

    pub fn root_hash<CS>(
        &self,
    ) -> Result<Scalar<E>>
    where
        CS: ConstraintSystem<E>,
    {

    }
}
