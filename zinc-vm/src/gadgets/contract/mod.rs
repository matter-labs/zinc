pub mod storage;

use num_bigint::BigInt;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::sha256::sha256;

use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub const ROOT_HASH_TRUNCATED_BITS: usize = 248;

pub trait MerkleTreeHasher<E: IEngine>: Default {
    fn hash_width(&self) -> usize;

    fn leaf_value_hash<CS>(
        &self,
        cs: CS,
        leaf_value: &[Scalar<E>],
    ) -> Result<Vec<Boolean>, RuntimeError>
    where
        CS: ConstraintSystem<E>;

    fn node_hash<CS>(
        &self,
        cs: CS,
        left_node: &[Boolean],
        right_node: &[Boolean],
    ) -> Result<Vec<Boolean>, RuntimeError>
    where
        CS: ConstraintSystem<E>;
}

#[derive(Default)]
pub struct Sha256Hasher;

impl<E: IEngine> MerkleTreeHasher<E> for Sha256Hasher {
    fn hash_width(&self) -> usize {
        256
    }

    fn leaf_value_hash<CS>(
        &self,
        mut cs: CS,
        leaf_value: &[Scalar<E>],
    ) -> Result<Vec<Boolean>, RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let mut preimage = Vec::new();

        for (index, field) in leaf_value.iter().enumerate() {
            let mut field_bits = field.to_expression::<CS>().into_bits_le_strict(
                cs.namespace(|| format!("{} field of leaf value to bits", index)),
            )?;
            field_bits.resize(zinc_const::BITLENGTH_FIELD_PADDED, Boolean::Constant(false));

            preimage.append(&mut field_bits);
        }

        Ok(sha256(cs.namespace(|| "sha256"), &preimage)?)
    }

    fn node_hash<CS>(
        &self,
        mut cs: CS,
        left_node: &[Boolean],
        right_node: &[Boolean],
    ) -> Result<Vec<Boolean>, RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        if left_node.len() != 256 || right_node.len() != 256 {
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

pub struct MerkleTreeLeaf<E: IEngine> {
    pub leaf_value: Vec<Option<Scalar<E>>>,
    pub leaf_value_hash: Vec<Option<bool>>,
    pub authentication_path: Vec<Vec<Option<bool>>>,
}

pub trait MerkleTreeStorage<E: IEngine> {
    /// Returns depth of merkle tree
    fn depth(&self) -> usize;

    /// Loads root hash
    fn root_hash(&self) -> Option<E::Fr>;

    /// Loads leaf value with authentication path
    fn load(&self, index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>, RuntimeError>;

    /// Stores value to storage, returns previous leaf value with authentication path
    fn store(
        &mut self,
        index: &Option<BigInt>,
        value: &[Option<Scalar<E>>],
    ) -> Result<MerkleTreeLeaf<E>, RuntimeError>;
}
