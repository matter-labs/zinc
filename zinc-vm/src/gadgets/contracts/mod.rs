mod storage;

pub use self::storage::*;

pub mod merkle_tree_storage {
    use primitive_types::U256;
    use crate::{Engine, Result};

    pub struct MerkleTreeLeaf<E: Engine> {
        pub leaf_value: E::Fr,
        pub authentication_path: Vec<E::Fr>,
    }

    pub trait MerkleTreeStorage<E: Engine> {
        const SHA256_TRUNCATED_BITLENGTH: usize = 248usize;

        /// Returns depth of merkle tree
        fn depth(&self) -> usize;

        /// Loads root hash
        fn root_hash(&self) -> Result<E::Fr>;

        /// Loads leaf value with authentication path
        fn load(&self, index: U256) -> Result<MerkleTreeLeaf<E>>;

        /// Stores value to storage, returns new merkle root and previous leaf value with authentication path
        fn store(&mut self, index: U256, value: &E::Fr) -> Result<(E::Fr, MerkleTreeLeaf<E>)>;
    }
}