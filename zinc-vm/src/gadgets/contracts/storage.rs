use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
use crate::gadgets::{IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use crate::stdlib::bits::signed_to_bits;
use crate::core::RuntimeError;
use num_integer::nth_root;

pub struct StorageGadget<'a, E: Engine, S: MerkleTreeStorage<E>> {
    storage: &'a S,
    root_hash: Scalar<E>,
}

impl<'a, E: Engine, S: MerkleTreeStorage<E>> StorageGadget<'a, E, S> {
    pub fn new<CS>(
        mut cs: CS,
        storage: &'a S,
    ) -> Result<Self>
    where
        CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage.root_hash()?;
        let root_hash_variable = cs
            .alloc_input(|| "root hash variable", || Ok(root_hash_value))
            .map_err(RuntimeError::SynthesisError)?;
        let root_hash = Scalar :: <E> :: new_unchecked_variable(
            Some(root_hash_value),
            root_hash_variable,
            ScalarType::Field,
        );
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
        panic!("esv");
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
        panic!("esv");
    }

    pub fn root_hash<CS>(
        &self,
    ) -> Result<Scalar<E>>
    where
        CS: ConstraintSystem<E>,
    {
        panic!("esv");
    }
}
