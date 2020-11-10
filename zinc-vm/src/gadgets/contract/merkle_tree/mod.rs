pub mod allocated_leaf;
pub mod hasher;

use num::BigInt;

use crate::core::contract::storage::leaf::Leaf;
use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::leaf::LeafVariant;
use crate::error::Error;
use crate::IEngine;

pub trait IMerkleTree<E: IEngine> {
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
    /// Returns the Merkle tree root hash.
    ///
    fn root_hash(&self) -> E::Fr;

    ///
    /// Returns the depth of the Merkle tree.
    ///
    fn depth(&self) -> usize;
}
