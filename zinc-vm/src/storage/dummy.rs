use crate::gadgets::merkle_tree_storage::{
    MerkleTreeLeaf, MerkleTreeStorage, ROOT_HASH_TRUNCATED_BITS,
};
use crate::{Engine, Result};
use ff::{PrimeField, PrimeFieldRepr};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use sha2::Digest;
use sha2::Sha256;

fn sha256<E: Engine>(preimage: Vec<u8>) -> Vec<u8> {
    let mut h = Sha256::new();
    h.input(&preimage);
    let result = h.result();

    result.as_slice().to_vec()
}

fn sha256_of_concat<E: Engine>(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    sha256::<E>([left.as_slice(), right.as_slice()].concat())
}

fn leaf_value_hash<E: Engine>(leaf_value: Vec<E::Fr>) -> Vec<u8> {
    let mut res = vec![];
    for field in leaf_value {
        let mut field_vec = vec![];
        field.into_repr().write_le(&mut field_vec).unwrap();
        field_vec.resize(256 / 8, 0);

        let mut field_vec_be = vec![];
        for i in field_vec {
            let mut cur_byte: u8 = 0;
            for j in 0..8 {
                cur_byte <<= 1;
                cur_byte += (i >> j) & 1u8;
            }
            field_vec_be.push(cur_byte);
        }

        res.append(&mut field_vec_be);
    }
    sha256::<E>(res)
}

pub struct DummyStorage<E: Engine> {
    depth: usize,
    tree: Vec<Vec<u8>>,
    leaf_values: Vec<Vec<E::Fr>>,
}

impl<E: Engine> DummyStorage<E> {
    fn rebuild_tree(&mut self) {
        for i in (1..(1 << (self.depth + 1))).rev() {
            if i < (1 << self.depth) {
                self.tree[i] =
                    sha256_of_concat::<E>(self.tree[i * 2].clone(), self.tree[i * 2 + 1].clone());
            } else {
                self.tree[i] =
                    leaf_value_hash::<E>(self.leaf_values[i - (1 << self.depth)].clone());
            }
        }
    }

    pub fn new(depth: usize) -> Self {
        let mut result = Self {
            depth,
            tree: vec![vec![]; 1 << (depth + 1)],
            leaf_values: vec![vec![]; 1 << depth],
        };

        result.rebuild_tree();

        result
    }
}

impl<E: Engine> MerkleTreeStorage<E> for DummyStorage<E> {
    fn depth(&self) -> usize {
        self.depth
    }

    fn root_hash(&self) -> Option<E::Fr> {
        let mut hash_as_buf = self.tree[1].clone();

        hash_as_buf.truncate(ROOT_HASH_TRUNCATED_BITS / 8);
        hash_as_buf.resize(256 / 8, 0);

        let mut hash_le = vec![];
        for i in &hash_as_buf {
            let mut cur_byte: u8 = 0;
            for j in 0..8 {
                cur_byte <<= 1;
                cur_byte += (i >> j) & 1u8;
            }
            hash_le.push(cur_byte);
        }

        let mut hash_repr = <E::Fr as PrimeField>::Repr::default();
        hash_repr.read_le(hash_le.as_slice()).unwrap();
        E::Fr::from_repr(hash_repr).ok()
    }

    fn load(&self, index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>> {
        let index = index.as_ref().unwrap();

        let index = index.to_usize().unwrap();

        let mut result = MerkleTreeLeaf::<E> {
            leaf_value: self.leaf_values[index]
                .iter()
                .map(|field| Some(*field))
                .collect(),
            leaf_value_hash: {
                let mut hash = vec![];
                for i in leaf_value_hash::<E>(self.leaf_values[index].clone()) {
                    for j in (0..8).rev() {
                        hash.push(Some(((i >> j) & 1u8) == 1u8))
                    }
                }
                hash
            },
            authentication_path: vec![],
        };

        let mut cur_node = 1;
        for i in (0..self.depth).rev() {
            let next = cur_node * 2 + ((index >> i) & 1usize);
            let mut cur_auth_path_node_hash = vec![];
            for i in &self.tree[next ^ 1usize] {
                for j in (0..8).rev() {
                    cur_auth_path_node_hash.push(Some(((i >> j) & 1u8) == 1u8));
                }
            }
            result.authentication_path.push(cur_auth_path_node_hash);

            cur_node = next;
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
            leaf_value_hash: {
                let mut hash = vec![];
                for i in leaf_value_hash::<E>(self.leaf_values[index].clone()) {
                    for j in (0..8).rev() {
                        hash.push(Some(((i >> j) & 1u8) == 1u8))
                    }
                }
                hash
            },
            authentication_path: vec![],
        };

        let mut cur_node = 1;
        for i in (0..self.depth).rev() {
            let next = cur_node * 2 + ((index >> i) & 1usize);
            let mut cur_auth_path_node_hash = vec![];
            for i in &self.tree[next ^ 1usize] {
                for j in (0..8).rev() {
                    cur_auth_path_node_hash.push(Some(((i >> j) & 1u8) == 1u8));
                }
            }
            result.authentication_path.push(cur_auth_path_node_hash);

            cur_node = next;
        }

        result.authentication_path.reverse();

        self.leaf_values[index] = value.to_vec();
        self.rebuild_tree();

        Ok(result)
    }
}
