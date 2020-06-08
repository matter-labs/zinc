use std::marker::PhantomData;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::contract::merkle_tree::hasher::IHasher as IMerkleTreeHasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct StorageGadget<E: IEngine, S: IMerkleTree<E>, H: IMerkleTreeHasher<E>> {
    storage: S,
    root_hash: Scalar<E>,
    _pd: PhantomData<H>,
}

fn alloc_leaf_fields<E, CS>(
    mut cs: CS,
    leaf_value: &[Option<Scalar<E>>],
) -> Result<Vec<Scalar<E>>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let mut leaf_fields = Vec::new();
    for (index, scalar) in leaf_value
        .iter()
        .filter_map(|scalar| scalar.as_ref())
        .enumerate()
    {
        if let Ok(fr) = scalar.grab_value() {
            let scalar_type = scalar.get_type();

            let field_allocated_num = AllocatedNum::alloc(
                cs.namespace(|| format!("leaf value: {} field", index)),
                || Ok(fr),
            )?;
            let scalar = Scalar::<E>::new_unchecked_variable(
                field_allocated_num.get_value(),
                field_allocated_num.get_variable(),
                scalar_type,
            );
            leaf_fields.push(scalar);
        }
    }

    Ok(leaf_fields)
}

fn alloc_leaf_hash<E, CS>(
    mut cs: CS,
    leaf_hash_value: &[Option<bool>],
) -> Result<Vec<Boolean>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let mut leaf_hash = Vec::new();
    for (bit_id, bit_value) in leaf_hash_value.iter().enumerate() {
        leaf_hash.push(Boolean::from(AllocatedBit::alloc(
            cs.namespace(|| format!("{} bit of leaf hash", bit_id)),
            *bit_value,
        )?));
    }

    Ok(leaf_hash)
}

fn alloc_authentication_path<E, CS>(
    mut cs: CS,
    depth: usize,
    authentication_path_value: &[Vec<Option<bool>>],
) -> Result<Vec<Vec<Scalar<E>>>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    assert_eq!(authentication_path_value.len(), depth);

    let mut authentication_path = Vec::new();
    for (index, hash_bits) in authentication_path_value.iter().enumerate() {
        let mut node_hash = Vec::new();
        for (bit_id, bit) in hash_bits.iter().enumerate() {
            let bit_boolean = Boolean::from(AllocatedBit::alloc(
                cs.namespace(|| {
                    format!(
                        "{} bit of node hash of authentication path (deep equals {})",
                        bit_id,
                        depth - index
                    )
                }),
                *bit,
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
        authentication_path.push(node_hash);
    }

    Ok(authentication_path)
}

enum AllocatedLeaf<E: IEngine> {
    LeafFields(Vec<Scalar<E>>),
    LeafHash(Vec<Boolean>),
}

/// Enforcing single leaf of merkle tree
/// Returns root hash variable
///
/// **Note**: index bits are in **little-endian**.
fn enforce_merkle_tree_path<E, CS, H>(
    mut cs: CS,
    depth: usize,
    hasher: &H,
    index_bits: &[Scalar<E>],
    leaf: &AllocatedLeaf<E>,
    authentication_path: &[Vec<Scalar<E>>],
) -> Result<Scalar<E>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
    H: IMerkleTreeHasher<E>,
{
    assert_eq!(index_bits.len(), depth);
    assert_eq!(authentication_path.len(), depth);
    for node_hash in authentication_path {
        assert_eq!(node_hash.len(), hasher.hash_width());
    }

    let mut current_hash = match leaf {
        AllocatedLeaf::LeafFields(leaf_value) => {
            hasher.leaf_value_hash(cs.namespace(|| "leaf value hash"), leaf_value)?
        }
        AllocatedLeaf::LeafHash(leaf_hash) => leaf_hash.clone(),
    };

    for (index, (node_hash, index_bit)) in authentication_path.iter().zip(index_bits).enumerate() {
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

    root_hash_bits.truncate(zinc_const::BITLENGTH_SHA256_HASH_TRUNCATED);

    Ok(Scalar::<E>::from(AllocatedNum::<E>::pack_bits_to_element(
        cs.namespace(|| "pack root hash bits into AllocatedNum"),
        &root_hash_bits,
    )?))
}

impl<E, S, H> StorageGadget<E, S, H>
where
    E: IEngine,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    pub fn new<CS>(mut cs: CS, storage: S) -> Result<Self, SynthesisError>
    where
        CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage
            .root_hash()
            .ok_or(SynthesisError::AssignmentMissing)?;
        let root_hash_variable = cs.alloc(|| "root hash variable", || Ok(root_hash_value))?;
        let root_hash = Scalar::<E>::new_unchecked_variable(
            Some(root_hash_value),
            root_hash_variable,
            ScalarType::Field,
        );

        Ok(StorageGadget {
            storage,
            root_hash,
            _pd: PhantomData,
        })
    }

    pub fn load<CS>(
        &self,
        mut cs: CS,
        size: usize,
        index: &Scalar<E>,
    ) -> Result<Vec<Scalar<E>>, RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let index = index
            .get_value()
            .map(|field| gadgets::scalar::fr_bigint::fr_to_bigint(&field, false));
        let merkle_tree_leaf = self.storage.load(&index)?;

        let leaf_fields = alloc_leaf_fields(
            cs.namespace(|| "alloc leaf fields"),
            &merkle_tree_leaf.leaf_value,
        )?;

        let authentication_path = alloc_authentication_path(
            cs.namespace(|| "alloc authentication path"),
            depth,
            &merkle_tree_leaf.authentication_path,
        )?;

        if leaf_fields.len() != size {
            return Err(RuntimeError::AssertionError(
                "Incorrect number of slot fields returned from storage".into(),
            ));
        }

        let authorized_root_hash = enforce_merkle_tree_path(
            cs.namespace(|| "enforce merkle tree path"),
            depth,
            &H::default(),
            &index_bits,
            &AllocatedLeaf::LeafFields(leaf_fields.clone()),
            &authentication_path,
        )?;

        let root_hash_condition = gadgets::comparison::equals(
            cs.namespace(|| "root hash equals to stored"),
            &authorized_root_hash,
            &self.root_hash,
        )?
        .to_boolean(cs.namespace(|| "root hash equals to stored to boolean"))?;
        Boolean::enforce_equal(
            cs.namespace(|| "enforcing that root hash equals to stored"),
            &root_hash_condition,
            &Boolean::Constant(true),
        )?;

        Ok(leaf_fields)
    }

    pub fn store<CS>(
        &mut self,
        mut cs: CS,
        index: &Scalar<E>,
        value: &[Scalar<E>],
    ) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let merkle_tree_leaf = self.storage.store(
            &index
                .get_value()
                .map(|field| gadgets::scalar::fr_bigint::fr_to_bigint(&field, false)),
            &value
                .iter()
                .cloned()
                .map(Option::Some)
                .collect::<Vec<Option<Scalar<E>>>>(),
        )?;

        let leaf_hash = alloc_leaf_hash(
            cs.namespace(|| "alloc leaf hash"),
            &merkle_tree_leaf.leaf_value_hash,
        )?;

        let authentication_path = alloc_authentication_path(
            cs.namespace(|| "alloc authentication path"),
            depth,
            &merkle_tree_leaf.authentication_path,
        )?;

        let authorized_root_hash = enforce_merkle_tree_path(
            cs.namespace(|| "enforce merkle tree path (loading value)"),
            depth,
            &H::default(),
            &index_bits,
            &AllocatedLeaf::LeafHash(leaf_hash),
            &authentication_path,
        )?;

        let root_hash_condition = gadgets::comparison::equals(
            cs.namespace(|| "root hash equals to stored"),
            &authorized_root_hash,
            &self.root_hash,
        )?
        .to_boolean(cs.namespace(|| "root hash equals to stored to boolean"))?;
        Boolean::enforce_equal(
            cs.namespace(|| "enforcing that root hash equals to stored"),
            &root_hash_condition,
            &Boolean::Constant(true),
        )?;

        self.root_hash = enforce_merkle_tree_path(
            cs.namespace(|| "enforce merkle tree path (storing value)"),
            depth,
            &H::default(),
            &index_bits,
            &AllocatedLeaf::LeafFields(value.to_vec()),
            &authentication_path,
        )?;

        Ok(())
    }

    pub fn root_hash(&self) -> Result<Scalar<E>, RuntimeError> {
        Ok(self.root_hash.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::StorageGadget;

    use rand::Rng;
    use rand::SeedableRng;
    use rand::XorShiftRng;

    use ff::PrimeField;
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::num::AllocatedNum;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use pairing::bn256::Bn256;
    use pairing::bn256::Fr;

    use zinc_bytecode::DataType;
    use zinc_bytecode::ScalarType;

    use crate::core::contract::storage::dummy::Storage as DummyStorage;
    use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
    use crate::gadgets::scalar::Scalar;

    #[ignore]
    #[test]
    fn test_storage_gadget_small() {
        const STORAGE_ELEMENT_COUNT: usize = 16;

        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        let mut cs = TestConstraintSystem::<Bn256>::new();

        let storage_test_dummy =
            DummyStorage::<Bn256>::new(vec![
                DataType::Scalar(ScalarType::Field);
                STORAGE_ELEMENT_COUNT
            ]);

        let mut storage_gadget = StorageGadget::<_, _, Sha256Hasher>::new(
            cs.namespace(|| "gadget creation"),
            storage_test_dummy,
        )
        .unwrap();

        for i in 0..STORAGE_ELEMENT_COUNT {
            let scalar = Scalar::<Bn256>::from(
                AllocatedNum::alloc(
                    cs.namespace(|| format!("variable :: index({}); field index({})", i, 1)),
                    || Ok(rng.gen()),
                )
                .unwrap(),
            );
            let fr = scalar.get_value().unwrap();

            storage_gadget
                .store(
                    cs.namespace(|| format!("store :: index({})", i)),
                    &Scalar::<Bn256>::new_constant_fr(
                        Fr::from_str(&i.to_string()).unwrap(),
                        ScalarType::Field,
                    ),
                    &[scalar],
                )
                .unwrap();

            let loaded_fr = storage_gadget
                .load(
                    cs.namespace(|| format!("load :: index({})", i)),
                    1,
                    &Scalar::<Bn256>::new_constant_fr(
                        Fr::from_str(&i.to_string()).unwrap(),
                        ScalarType::Field,
                    ),
                )
                .unwrap()
                .remove(0)
                .get_value()
                .unwrap();

            assert_eq!(loaded_fr, fr);
        }

        assert!(cs.is_satisfied());
    }
}
