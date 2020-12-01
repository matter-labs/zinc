pub mod allocated_leaf;
pub mod hasher;

use num::BigInt;

use crate::core::contract::storage::leaf::Leaf;
use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::leaf::LeafVariant;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub trait IMerkleTree<E: IEngine> {
    ///
    /// Creates a contract storage from the evaluation stack.
    ///
    fn from_evaluation_stack(
        field_types: Vec<zinc_types::ContractFieldType>,
        values: Vec<Scalar<E>>,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Creates a contract storage from the metadata representation.
    ///
    fn from_build(
        field_types: Vec<zinc_types::ContractFieldType>,
        value: zinc_types::Value,
    ) -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Loads a leaf value with authentication path at `index`.
    ///
    fn load(&self, index: BigInt) -> Result<Leaf<E>, Error>;

    ///
    /// Stores `values` to storage, returns the previous leaf value with authentication path.
    ///
    fn store(&mut self, index: BigInt, values: LeafVariant<E>) -> Result<(), Error>;

    ///
    /// Returns the storage values.
    ///
    fn into_values(self) -> Vec<LeafOutput>;

    ///
    /// Returns the storage field types.
    ///
    fn types(&self) -> &[zinc_types::ContractFieldType];

    ///
    /// Returns the Merkle tree root hash.
    ///
    fn root_hash(&self) -> E::Fr;

    ///
    /// Returns the depth of the Merkle tree.
    ///
    fn depth(&self) -> usize;
}
