mod storage;

pub use self::storage::*;

pub mod merkle_tree_storage {
    use crate::{Engine, Result};
    use num_bigint::BigInt;

    pub struct MerkleTreeLeaf<E: Engine> {
        pub leaf_value: Vec<Option<E::Fr>>,
        pub authentication_path: Vec<Vec<Option<bool>>>,
    }

    pub const ROOT_HASH_TRUNCATED_BITS: usize = 248;

    pub trait MerkleTreeStorage<E: Engine> {
        /// Returns depth of merkle tree
        fn depth(&self) -> usize;

        /// Loads root hash
        fn root_hash(&self) -> Result<E::Fr>;

        /// Loads leaf value with authentication path
        fn load(&self, index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>>;

        /// Stores value to storage, returns previous leaf value with authentication path
        fn store(
            &mut self,
            index: &Option<BigInt>,
            value: &[Option<E::Fr>],
        ) -> Result<MerkleTreeLeaf<E>>;
    }
}
