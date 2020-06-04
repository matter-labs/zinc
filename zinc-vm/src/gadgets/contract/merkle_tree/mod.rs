pub mod hasher;
pub mod leaf;

use num_bigint::BigInt;

use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

use self::leaf::Leaf;

pub trait IMerkleTree<E: IEngine> {
    /// Returns depth of merkle tree
    fn depth(&self) -> usize;

    /// Returns root hash
    fn root_hash(&self) -> Option<E::Fr>;

    /// Loads leaf value with authentication path
    fn load(&self, index: &Option<BigInt>) -> Result<Leaf<E>, RuntimeError>;

    /// Stores value to storage, returns previous leaf value with authentication path
    fn store(
        &mut self,
        index: &Option<BigInt>,
        value: &[Option<Scalar<E>>],
    ) -> Result<Leaf<E>, RuntimeError>;
}
