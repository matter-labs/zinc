mod storage;

pub use self::storage::*;

pub mod merkle_tree_storage {
    use num_bigint::{BigInt, Sign};
    use crate::{Engine, Result};

    pub struct MerkleTreeLeaf<E: Engine> {
        pub leaf_value: Vec<E::Fr>,
        pub authentication_path: Vec<Vec<bool>>,
    }

    pub trait MerkleTreeStorage<E: Engine> {
        /// Returns depth of merkle tree
        fn depth(&self) -> Result<usize>;

        /// Loads root hash
        fn root_hash(&self) -> Result<E::Fr>;

        /// Loads leaf value with authentication path
        fn load(&self, index: &BigInt) -> Result<MerkleTreeLeaf<E>>;

        /// Stores value to storage, returns new merkle root and previous leaf value with authentication path
        fn store(
            &mut self,
            index: &BigInt,
            value: &Vec<E::Fr>,
        ) -> Result<(E::Fr, MerkleTreeLeaf<E>)>;
    }
}
