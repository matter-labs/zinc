mod storage;

pub use self::storage::*;

pub mod merkle_tree_storage {
    use crate::{Engine, Result};
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::boolean::Boolean;
    use franklin_crypto::circuit::sha256::sha256;
    use num_bigint::BigInt;

    pub mod merkle_tree_hash {
        use super::*;

        pub trait MerkleTreeHash<E: Engine>: Sized {
            fn execute<CS>(&self, cs: CS, preimage: &[Boolean]) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>;
        }

        pub struct Sha256Hasher;

        impl<E: Engine> MerkleTreeHash<E> for Sha256Hasher {
            fn execute<CS>(&self, mut cs: CS, preimage: &[Boolean]) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>,
            {
                let mut preimage = preimage
                    .iter()
                    .map(|boolean| boolean.clone())
                    .collect::<Vec<Boolean>>();
                while (preimage.len() % 8 != 0) {
                    preimage.push(Boolean::Constant(false));
                }
                Ok(sha256(cs.namespace(|| "sha256 hash"), &preimage)?)
            }
        }
    }

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
