use crate::gadgets::comparison::eq;
use crate::gadgets::conditional_select;
use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeStorage, ROOT_HASH_TRUNCATED_BITS};
use crate::gadgets::utils::fr_to_bigint_unsigned;
use crate::gadgets::{Scalar, ScalarType};
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::sha256::sha256;
use franklin_crypto::circuit::Assignment;

pub struct StorageGadget<E: Engine, S: MerkleTreeStorage<E>> {
    storage: S,
    root_hash: Scalar<E>,
}

fn alloc_leaf_and_authentication_path<E, CS>(
    mut cs: CS,
    depth: usize,
    leaf_value: &Vec<Option<E::Fr>>,
    authentication_path_value: &Vec<Vec<Option<bool>>>,
) -> Result<(Vec<Scalar<E>>, Vec<Vec<Scalar<E>>>)>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let mut authentication_path = Vec::new();

    for (index, hash_bits) in authentication_path_value.iter().enumerate() {
        let mut vertex_hash = Vec::new();
        for (bit_id, bit) in hash_bits.iter().enumerate() {
            let bit_boolean = Boolean::from(AllocatedBit::alloc(
                cs.namespace(|| {
                    format!(
                        "{} bit of vertex hash of authentication path (deep equals {})",
                        bit_id,
                        depth - index
                    )
                }),
                *bit,
            )?);
            let bit_scalar = Scalar::<E>::from_boolean(
                cs.namespace(|| {
                    format!(
                        "{} bit of vertex hash of authentication path to scalar (deep equals {})",
                        bit_id,
                        depth - index
                    )
                }),
                bit_boolean,
            )?;
            vertex_hash.push(bit_scalar);
        }
        authentication_path.push(vertex_hash);
    }

    let mut leaf = Vec::new();
    for (index, field) in leaf_value.iter().enumerate() {
        let field_allocated_num = AllocatedNum::alloc(
            cs.namespace(|| format!("leaf value: field with index {}", index)),
            || Ok(field.grab()?),
        )?;
        leaf.push(Scalar::<E>::from(field_allocated_num));
    }

    Ok((leaf, authentication_path))
}

fn enfoce_leaf_value_hash<E, CS>(mut cs: CS, leaf_value: &[Scalar<E>]) -> Result<Vec<Boolean>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let mut leaf_value_hash: Vec<Boolean> = Vec::new();

    for (index, field) in leaf_value.iter().enumerate() {
        let mut field_bits = field.to_expression::<CS>().into_bits_le_strict(
            cs.namespace(|| format!("{} field of leaf value to bits", index)),
        )?;
        field_bits.resize(256, Boolean::Constant(false));

        let mut preimage = Vec::new();
        preimage.append(&mut leaf_value_hash);
        preimage.append(&mut field_bits);

        leaf_value_hash = sha256(
            cs.namespace(|| {
                format!(
                    "hash of previous leaf value hash with next field (index {})",
                    index
                )
            }),
            &preimage,
        )?;
    }

    Ok(leaf_value_hash)
}

/// Enforcing single leaf of merkle tree
/// Returns root hash variable
///
/// **Note**: index bits are in **little-endian**.
fn enforce_merkle_tree_path<E, CS>(
    mut cs: CS,
    depth: usize,
    index_bits: &[Scalar<E>],
    leaf_value: &[Scalar<E>],
    authentication_path: &Vec<Vec<Scalar<E>>>,
) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    assert!(leaf_value.len() != 0);
    assert_eq!(index_bits.len(), depth);
    assert_eq!(authentication_path.len(), depth);
    for i in authentication_path {
        assert_eq!(i.len(), 256);
    }

    let mut current_hash = enfoce_leaf_value_hash(cs.namespace(|| "leaf value hash"), leaf_value)?;

    for (index, (vertex_hash, index_bit)) in authentication_path.iter().zip(index_bits).enumerate()
    {
        let mut hash_preimage = vec![Boolean::Constant(false); 512];

        for (bit_id, (current_hash_bit, vertex_hash_bit_scalar)) in
            current_hash.into_iter().zip(vertex_hash).enumerate()
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

            hash_preimage[bit_id] = conditional_select(
                    cs.namespace(|| {
                        format!(
                            "vertex hash preimage: left part conditional select: {} bit (deep equals {})",
                            bit_id,
                            depth - 1 - index
                        )
                    }),
                    index_bit,
                    &vertex_hash_bit_scalar,
                    &current_hash_bit_scalar,
                )?.to_boolean(cs.namespace(|| format!("vertex hash preimage: left part to boolean: {} bit (deep equals {})", bit_id, depth - 1 - index)))?;

            hash_preimage[256 + bit_id] = conditional_select(
                    cs.namespace(|| {
                        format!(
                            "vertex hash preimage: right part conditional select: {} bit (deep equals {})",
                            256 + bit_id,
                            depth - 1 - index
                        )
                    }),
                    index_bit,
                    &current_hash_bit_scalar,
                    &vertex_hash_bit_scalar,
                )?.to_boolean(cs.namespace(|| format!("vertex hash preimage: right part to boolean: {} bit (deep equals {})", bit_id, depth - 1 - index)))?;
        }

        current_hash = sha256(
            cs.namespace(|| format!("vertex hash (deep equals {})", depth - 1 - index)),
            &hash_preimage,
        )?;
    }

    let mut root_hash_bits = current_hash;

    root_hash_bits.truncate(ROOT_HASH_TRUNCATED_BITS);

    Ok(Scalar::<E>::from(AllocatedNum::<E>::pack_bits_to_element(
        cs.namespace(|| "pack root hash bits into AllocatedNum"),
        &root_hash_bits,
    )?))
}

impl<E: Engine, S: MerkleTreeStorage<E>> StorageGadget<E, S> {
    pub fn new<CS>(mut cs: CS, storage: S) -> Result<Self>
    where
        CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage.root_hash()?;
        let root_hash_variable = cs.alloc_input(|| "root hash variable", || Ok(root_hash_value))?;
        let root_hash = Scalar::<E>::new_unchecked_variable(
            Some(root_hash_value),
            root_hash_variable,
            ScalarType::Field,
        );

        Ok(StorageGadget {
            storage: storage,
            root_hash: root_hash,
        })
    }

    pub fn load<CS>(&self, mut cs: CS, index: &Scalar<E>) -> Result<Vec<Scalar<E>>>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let merkle_tree_leaf = self
            .storage
            .load(&index.get_value().map(|field| fr_to_bigint_unsigned(&field)))?;

        let (leaf, authentication_path) = alloc_leaf_and_authentication_path(
            cs.namespace(|| "alloc leaf and authentication path"),
            depth,
            &merkle_tree_leaf.leaf_value,
            &merkle_tree_leaf.authentication_path,
        )?;

        let authorized_root_hash = enforce_merkle_tree_path(
            cs.namespace(|| "enforce merkle tree path"),
            depth,
            &index_bits,
            &leaf,
            &authentication_path,
        )?;

        let root_hash_condition = eq(
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

        Ok(leaf)
    }

    pub fn store<CS>(&mut self, mut cs: CS, index: &Scalar<E>, value: &[Scalar<E>]) -> Result<()>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let fields = value.len();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let merkle_tree_leaf = self.storage.store(
            &index.get_value().map(|field| fr_to_bigint_unsigned(&field)),
            &value
                .iter()
                .map(|field| field.get_value())
                .collect::<Vec<Option<E::Fr>>>(),
        )?;

        let (leaf, authentication_path) = alloc_leaf_and_authentication_path(
            cs.namespace(|| "alloc leaf and authentication path"),
            depth,
            &merkle_tree_leaf.leaf_value,
            &merkle_tree_leaf.authentication_path,
        )?;

        let authorized_root_hash = enforce_merkle_tree_path(
            cs.namespace(|| "enforce merkle tree path (loading value)"),
            depth,
            &index_bits,
            &leaf,
            &authentication_path,
        )?;

        let root_hash_condition = eq(
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
            &index_bits,
            &value,
            &authentication_path,
        )?;

        Ok(())
    }

    pub fn root_hash<CS>(&self) -> Result<Scalar<E>>
    where
        CS: ConstraintSystem<E>,
    {
        Ok(self.root_hash.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::StorageGadget;
    use super::*;
    use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
    use crate::gadgets::{Scalar, ScalarType};
    use crate::{Engine, Result};
    use ff::{Field, PrimeField, PrimeFieldRepr};
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::num::AllocatedNum;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use num_bigint::BigInt;
    use num_traits::cast::ToPrimitive;
    use pairing::bn256::{Bn256, Fr};
    use rand::{Rng, SeedableRng, XorShiftRng};
    use sha2::{Digest, Sha256};

    mod storage_test_dummy {
        use super::*;

        fn sha256_of_concat<E: Engine>(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
            let mut preimage: Vec<u8> = vec![];
            preimage.append(&mut left.clone());
            preimage.append(&mut right.clone());
            let mut h = Sha256::new();
            h.input(&preimage);
            let result = h.result();

            result.as_slice().to_vec()
        }

        fn add_Fr_to_hash<E: Engine>(left: Vec<u8>, right_Fr: E::Fr) -> Vec<u8> {
            let mut right_buf = vec![];
            right_Fr.into_repr().write_le(&mut right_buf).unwrap();
            right_buf.resize(256 / 8, 0);

            let mut right = vec![];
            for i in right_buf {
                let mut cur_byte: u8 = 0;
                for j in (0..8) {
                    cur_byte <<= 1;
                    cur_byte += ((i >> j) & 1u8);
                }
                right.push(cur_byte);
            }

            sha256_of_concat::<E>(left, right)
        }

        fn sha256_of_leaf_value<E: Engine>(leaf_value: Vec<E::Fr>) -> Vec<u8> {
            let mut res = vec![];
            for i in leaf_value {
                res = add_Fr_to_hash::<E>(res, i);
            }
            res
        }

        pub struct StorageTestDummy<E: Engine> {
            depth: usize,
            tree: Vec<Vec<u8>>,
            leaf_values: Vec<Vec<E::Fr>>,
        }

        impl<E: Engine> StorageTestDummy<E> {
            fn rebuild_tree(&mut self) {
                for i in (1..(1 << (self.depth + 1))).rev() {
                    if (i < (1 << self.depth)) {
                        self.tree[i] = sha256_of_concat::<E>(
                            self.tree[i * 2].clone(),
                            self.tree[i * 2 + 1].clone(),
                        );
                    } else {
                        self.tree[i] = sha256_of_leaf_value::<E>(
                            self.leaf_values[i - (1 << self.depth)].clone(),
                        );
                    }
                }
            }

            pub fn new(depth: usize, fields: usize) -> Self {
                let mut result = Self {
                    depth: depth,
                    tree: vec![vec![]; 1 << (depth + 1)],
                    leaf_values: vec![vec![E::Fr::zero(); fields]; 1 << depth],
                };

                result.rebuild_tree();

                result
            }
        }

        impl<E: Engine> MerkleTreeStorage<E> for StorageTestDummy<E> {
            fn depth(&self) -> usize {
                self.depth
            }

            fn root_hash(&self) -> Result<E::Fr> {
                let mut hash_as_buf = self.tree[1].clone();

                hash_as_buf.truncate(ROOT_HASH_TRUNCATED_BITS / 8);
                hash_as_buf.resize(256 / 8, 0);

                let mut hash_le = vec![];
                for i in &hash_as_buf {
                    let mut cur_byte: u8 = 0;
                    for j in (0..8) {
                        cur_byte <<= 1;
                        cur_byte += ((i >> j) & 1u8);
                    }
                    hash_le.push(cur_byte);
                }

                let mut hash_repr = <E::Fr as PrimeField>::Repr::default();
                hash_repr.read_le(hash_le.as_slice()).unwrap();
                Ok(E::Fr::from_repr(hash_repr).unwrap())
            }

            fn load(&self, index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>> {
                let index = index.as_ref().unwrap();

                let index = index.to_usize().unwrap();

                let mut result = MerkleTreeLeaf::<E> {
                    leaf_value: self.leaf_values[index]
                        .iter()
                        .map(|field| Some(*field))
                        .collect(),
                    authentication_path: vec![],
                };

                let mut cur_vertex = 1;
                for i in (0..self.depth).rev() {
                    let next = cur_vertex * 2 + ((index >> i) & 1usize);
                    let mut cur_auth_path_vertex_hash = vec![];
                    for i in &self.tree[next ^ 1usize] {
                        for j in (0..8).rev() {
                            cur_auth_path_vertex_hash.push(Some(((i >> j) & 1u8) == 1u8));
                        }
                    }
                    result.authentication_path.push(cur_auth_path_vertex_hash);

                    cur_vertex = next;
                }

                result.authentication_path.reverse();

                Ok(result)
            }

            fn store(
                &mut self,
                index: &Option<BigInt>,
                value: &[Option<E::Fr>],
            ) -> Result<MerkleTreeLeaf<E>> {
                let index = index.as_ref().unwrap();
                let value = &value
                    .iter()
                    .map(|field| field.unwrap())
                    .collect::<Vec<E::Fr>>();

                let index = index.to_usize().unwrap();

                let mut result = MerkleTreeLeaf::<E> {
                    leaf_value: self.leaf_values[index]
                        .iter()
                        .map(|field| Some(*field))
                        .collect(),
                    authentication_path: vec![],
                };

                let mut cur_vertex = 1;
                for i in (0..self.depth).rev() {
                    let next = cur_vertex * 2 + ((index >> i) & 1usize);
                    let mut cur_auth_path_vertex_hash = vec![];
                    for i in &self.tree[next ^ 1usize] {
                        for j in (0..8).rev() {
                            cur_auth_path_vertex_hash.push(Some(((i >> j) & 1u8) == 1u8));
                        }
                    }
                    result.authentication_path.push(cur_auth_path_vertex_hash);

                    cur_vertex = next;
                }

                result.authentication_path.reverse();

                self.leaf_values[index] = value.to_vec();

                self.rebuild_tree();

                Ok(result)
            }
        }
    }

    use storage_test_dummy::*;

    #[test]
    fn test_storage_gadget_small() {
        const DEPTH_OF_TEST_TREE: usize = 2;

        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        let mut cs = TestConstraintSystem::<Bn256>::new();

        let storage_test_dummy = StorageTestDummy::<Bn256>::new(DEPTH_OF_TEST_TREE, 1);

        let mut storage_gadget =
            StorageGadget::new(cs.namespace(|| "gadget creation"), storage_test_dummy).unwrap();

        for iter in (0..=1) {
            let mut cs = cs.namespace(|| format!("iter :: {}", iter));
            for i in (0..(1 << DEPTH_OF_TEST_TREE)) {
                let mut cur_vec = vec![];
                let fields = rng.gen::<usize>() % 2 + 1;
                for j in (0..fields) {
                    cur_vec.push(Scalar::<Bn256>::from(
                        AllocatedNum::alloc(
                            cs.namespace(|| {
                                format!("variable :: index({}); field index({})", i, j)
                            }),
                            || Ok(rng.gen()),
                        )
                        .unwrap(),
                    ));
                }

                storage_gadget
                    .store(
                        cs.namespace(|| format!("store :: index({})", i)),
                        &Scalar::<Bn256>::new_constant_fr(
                            Fr::from_str(&i.to_string()).unwrap(),
                            ScalarType::Field,
                        ),
                        &cur_vec,
                    )
                    .unwrap();

                let loaded_result = storage_gadget
                    .load(
                        cs.namespace(|| format!("load :: index({})", i)),
                        &Scalar::<Bn256>::new_constant_fr(
                            Fr::from_str(&i.to_string()).unwrap(),
                            ScalarType::Field,
                        ),
                    )
                    .unwrap();
                assert_eq!(
                    loaded_result
                        .iter()
                        .map(|scalar| scalar.get_value().unwrap())
                        .collect::<Vec<Fr>>(),
                    cur_vec
                        .iter()
                        .map(|scalar| scalar.get_value().unwrap())
                        .collect::<Vec<Fr>>()
                );
            }
        }

        assert!(cs.is_satisfied());
    }
}
