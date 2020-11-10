use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::sha256;

use crate::error::Error;
use crate::gadgets::contract::merkle_tree::hasher::IHasher as IMerkleTreeHasher;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Default)]
pub struct Hasher {}

impl<E: IEngine> IMerkleTreeHasher<E> for Hasher {
    fn hash_width(&self) -> usize {
        zinc_const::bitlength::SHA256_HASH
    }

    fn leaf_value_hash<CS>(
        &self,
        mut cs: CS,
        leaf_value: &[Scalar<E>],
    ) -> Result<Vec<Boolean>, Error>
    where
        CS: ConstraintSystem<E>,
    {
        let mut preimage = Vec::new();

        for (index, field) in leaf_value.iter().enumerate() {
            let mut field_bits = field.to_expression::<CS>().into_bits_le_strict(
                cs.namespace(|| format!("{} field of leaf value to bits", index)),
            )?;
            field_bits.resize(
                zinc_const::bitlength::FIELD_PADDED,
                Boolean::Constant(false),
            );

            preimage.append(&mut field_bits);
        }

        Ok(sha256::sha256(
            cs.namespace(|| "leaf_value_sha256"),
            &preimage,
        )?)
    }

    fn node_hash<CS>(
        &self,
        mut cs: CS,
        left_node: &[Boolean],
        right_node: &[Boolean],
    ) -> Result<Vec<Boolean>, Error>
    where
        CS: ConstraintSystem<E>,
    {
        if left_node.len() != zinc_const::bitlength::SHA256_HASH
            || right_node.len() != zinc_const::bitlength::SHA256_HASH
        {
            return Err(Error::RequireError("Incorrect node hash width".into()));
        }

        Ok(sha256::sha256(
            cs.namespace(|| "node_sha256"),
            &[left_node, right_node].concat(),
        )?)
    }
}
