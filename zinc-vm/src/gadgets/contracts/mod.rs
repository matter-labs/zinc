mod storage;

pub use self::storage::*;

pub mod merkle_tree_storage {
    use crate::core::RuntimeError;
    use crate::gadgets::Scalar;
    use crate::{Engine, Result};
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::boolean::Boolean;
    use franklin_crypto::circuit::sha256::sha256;
    use num_bigint::BigInt;

    pub mod merkle_tree_hash {
        use super::*;

        pub trait MerkleTreeHash<E: Engine>: Sized {
            fn leaf_value_hash<CS>(&self, cs: CS, leaf_value: &[Scalar<E>]) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>;

            fn node_hash<CS>(
                &self,
                cs: CS,
                left_node: &[Boolean],
                right_node: &[Boolean],
            ) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>;
        }

        pub struct Sha256Hasher;

        impl<E: Engine> MerkleTreeHash<E> for Sha256Hasher {
            fn leaf_value_hash<CS>(
                &self,
                mut cs: CS,
                leaf_value: &[Scalar<E>],
            ) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>,
            {
                let mut preimage = Vec::new();

                for (index, field) in leaf_value.iter().enumerate() {
                    let mut field_bits = field.to_expression::<CS>().into_bits_le_strict(
                        cs.namespace(|| format!("{} field of leaf value to bits", index)),
                    )?;
                    field_bits.resize(256, Boolean::Constant(false));

                    preimage.append(&mut field_bits);
                }

                Ok(sha256(cs.namespace(|| "sha256"), &preimage)?)
            }

            fn node_hash<CS>(
                &self,
                mut cs: CS,
                left_node: &[Boolean],
                right_node: &[Boolean],
            ) -> Result<Vec<Boolean>>
            where
                CS: ConstraintSystem<E>,
            {
                if (left_node.len() != 256 || right_node.len() != 256) {
                    return Err(RuntimeError::AssertionError(
                        "Incorrect node hash width".into(),
                    ));
                }
                Ok(sha256(
                    cs.namespace(|| "sha256"),
                    &[left_node, right_node].concat(),
                )?)
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
