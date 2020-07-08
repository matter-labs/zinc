pub mod allocated_leaf;
pub mod hasher;

use num_bigint::BigInt;

use crate::core::contract::storage::leaf::Leaf as StorageLeaf;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub trait IMerkleTree<E: IEngine> {
    ///
    /// Loads a leaf value with authentication path at `index`.
    ///
    fn load(&self, index: BigInt) -> Result<StorageLeaf<E>, RuntimeError>;

    ///
    /// Stores `values` to storage, returns the previous leaf value with authentication path.
    ///
    fn store(
        &mut self,
        index: BigInt,
        values: Vec<Scalar<E>>,
    ) -> Result<StorageLeaf<E>, RuntimeError>;

    ///
    /// Returns the storage values.
    ///
    fn into_values(self) -> Vec<Vec<BigInt>>;

    ///
    /// Returns the Merkle tree root hash.
    ///
    fn root_hash(&self) -> E::Fr;

    ///
    /// Returns the depth of the Merkle tree.
    ///
    fn depth(&self) -> usize;
}
