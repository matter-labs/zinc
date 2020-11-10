use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::Assignment;

use crate::error::Error;
use crate::gadgets;
use crate::gadgets::contract::merkle_tree::hasher::IHasher as IMerkleTreeHasher;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub enum AllocatedLeaf<E: IEngine> {
    LeafFields(Vec<Scalar<E>>),
    LeafHash(Vec<Boolean>),
}

impl<E: IEngine> AllocatedLeaf<E> {
    ///
    /// Enforcces single leaf of a merkle tree.
    /// Returns the root hash variable.
    ///
    /// Index bits are **little-endian**.
    ///
    pub fn enforce_merkle_tree_path<CS, H>(
        &self,
        mut cs: CS,
        depth: usize,
        hasher: &H,
        index_bits: &[Scalar<E>],
        authentication_path: &[Vec<Scalar<E>>],
    ) -> Result<Scalar<E>, Error>
    where
        CS: ConstraintSystem<E>,
        H: IMerkleTreeHasher<E>,
    {
        assert_eq!(index_bits.len(), depth);
        assert_eq!(authentication_path.len(), depth);
        for node_hash in authentication_path {
            assert_eq!(node_hash.len(), hasher.hash_width());
        }

        let mut current_hash = match self {
            AllocatedLeaf::LeafFields(leaf_value) => {
                hasher.leaf_value_hash(cs.namespace(|| "leaf value hash"), leaf_value)?
            }
            AllocatedLeaf::LeafHash(leaf_hash) => leaf_hash.clone(),
        };

        for (index, (node_hash, index_bit)) in
            authentication_path.iter().zip(index_bits).enumerate()
        {
            let mut left_node = Vec::new();
            let mut right_node = Vec::new();

            for (bit_id, (current_hash_bit, node_hash_bit_scalar)) in
                current_hash.into_iter().zip(node_hash).enumerate()
            {
                let current_hash_bit_scalar = Scalar::<E>::from_boolean(
                    cs.namespace(|| {
                        format!(
                            "{} bit of current hash to scalar (deep equals {})",
                            bit_id,
                            depth - index
                        )
                    }),
                    current_hash_bit,
                )?;

                left_node.push(
                    gadgets::select::conditional(
                        cs.namespace(|| {
                            format!(
                                "node hash preimage: left part conditional select: {} bit (deep equals {})",
                                bit_id,
                                depth - 1 - index,
                            )
                        }),
                        index_bit,
                        &node_hash_bit_scalar,
                        &current_hash_bit_scalar,
                    )?
                        .to_boolean(cs.namespace(|| {
                            format!(
                                "node hash preimage: left part to boolean: {} bit (deep equals {})",
                                bit_id,
                                depth - 1 - index,
                            )
                        }))?,
                );

                right_node.push(
                    gadgets::select::conditional(
                        cs.namespace(|| {
                            format!(
                                "node hash preimage: right part conditional select: {} bit (deep equals {})",
                                bit_id,
                                depth - 1 - index,
                            )
                        }),
                        index_bit,
                        &current_hash_bit_scalar,
                        &node_hash_bit_scalar,
                    )?
                        .to_boolean(cs.namespace(|| {
                            format!(
                                "node hash preimage: right part to boolean: {} bit (deep equals {})",
                                bit_id,
                                depth - 1 - index,
                            )
                        }))?,
                );
            }

            current_hash = hasher.node_hash(
                cs.namespace(|| format!("node hash (deep equals {})", depth - 1 - index)),
                &left_node,
                &right_node,
            )?;
        }

        let mut root_hash_bits = current_hash;

        root_hash_bits.truncate(zinc_const::bitlength::SHA256_HASH - zinc_const::bitlength::BYTE);

        Ok(Scalar::<E>::from(AllocatedNum::<E>::pack_bits_to_element(
            cs.namespace(|| "pack root hash bits into AllocatedNum"),
            &root_hash_bits,
        )?))
    }

    pub fn alloc_leaf_fields<CS>(
        mut cs: CS,
        leaf_value: Vec<Scalar<E>>,
    ) -> Result<Vec<Scalar<E>>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let mut leaf_fields = Vec::with_capacity(leaf_value.len());
        for (index, scalar) in leaf_value.into_iter().enumerate() {
            let r#type = scalar.get_type();
            let fr = scalar.get_value();

            let field_allocated_num = AllocatedNum::alloc(
                cs.namespace(|| format!("leaf value: {} field", index)),
                || Ok(fr.grab()?),
            )?;

            let mut scalar = Scalar::<E>::from(field_allocated_num);
            scalar = scalar.to_type_unchecked(r#type);
            leaf_fields.push(scalar);
        }
        Ok(leaf_fields)
    }

    pub fn alloc_leaf_hash<CS>(mut cs: CS, leaf_hash_value: &[bool]) -> Result<Vec<Boolean>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let mut leaf_hash = Vec::with_capacity(leaf_hash_value.len());
        for (bit_id, bit_value) in leaf_hash_value.iter().enumerate() {
            leaf_hash.push(Boolean::from(AllocatedBit::alloc(
                cs.namespace(|| format!("{} bit of leaf hash", bit_id)),
                Some(*bit_value),
            )?));
        }
        Ok(leaf_hash)
    }

    pub fn alloc_authentication_path<CS>(
        mut cs: CS,
        depth: usize,
        authentication_path: Vec<Vec<bool>>,
    ) -> Result<Vec<Vec<Scalar<E>>>, Error>
    where
        CS: ConstraintSystem<E>,
    {
        assert_eq!(authentication_path.len(), depth);

        let mut allocated_path = Vec::with_capacity(authentication_path.len());
        for (index, hash_bits) in authentication_path.into_iter().enumerate() {
            let mut node_hash = Vec::new();
            for (bit_id, bit) in hash_bits.into_iter().enumerate() {
                let bit_boolean = Boolean::from(AllocatedBit::alloc(
                    cs.namespace(|| {
                        format!(
                            "{} bit of node hash of authentication path (deep equals {})",
                            bit_id,
                            depth - index
                        )
                    }),
                    Some(bit),
                )?);
                let bit_scalar = Scalar::<E>::from_boolean(
                    cs.namespace(|| {
                        format!(
                            "{} bit of node hash of authentication path to scalar (deep equals {})",
                            bit_id,
                            depth - index
                        )
                    }),
                    bit_boolean,
                )?;
                node_hash.push(bit_scalar);
            }
            allocated_path.push(node_hash);
        }

        Ok(allocated_path)
    }
}
