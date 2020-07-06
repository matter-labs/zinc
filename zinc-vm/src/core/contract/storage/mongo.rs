use num_bigint::BigInt;
use num_traits::ToPrimitive;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;

use zinc_bytecode::DataType;

use crate::core::contract::storage::leaf::Leaf as StorageLeaf;
use crate::core::contract::storage::sha256;
use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage<E: IEngine> {
    hash_tree: Vec<Vec<u8>>,
    leaf_values: Vec<Vec<Scalar<E>>>,
    depth: usize,
}

impl<E: IEngine> Storage<E> {
    pub fn new(values: Vec<DataType>) -> Self {
        let depth = (values.len() as f64).log2().ceil() as usize;
        let hash_tree_size = 1 << (depth + 1);
        let leaf_values_count = 1 << depth;

        let mut result = Self {
            hash_tree: vec![vec![]; hash_tree_size],
            leaf_values: vec![vec![]; leaf_values_count],
            depth,
        };

        for (index, r#type) in values.into_iter().enumerate() {
            let values = r#type
                .into_flat_scalar_types()
                .into_iter()
                .map(|r#type| Scalar::<E>::new_constant_usize(0, r#type))
                .collect();
            result.leaf_values[index] = values;
        }
        result.update_tree();

        result
    }

    fn update_tree(&mut self) {
        let non_leaf_nodes_count = 1 << self.depth;

        for i in (1..self.hash_tree.len()).rev() {
            let is_leaf = i >= non_leaf_nodes_count;

            if is_leaf {
                self.hash_tree[i] = sha256::leaf_value_hash::<E>(
                    self.leaf_values[i - non_leaf_nodes_count].clone(),
                );
            } else {
                self.hash_tree[i] = sha256::sha256_of_concat::<E>(
                    &self.hash_tree[i * 2],
                    &self.hash_tree[i * 2 + 1],
                );
            }
        }
    }

    fn leaf_with_auth(&self, index: usize) -> StorageLeaf<E> {
        let mut authentication_path = Vec::with_capacity(self.depth);
        let mut current_node = 1;
        for i in (0..self.depth).rev() {
            let next_node = current_node * 2 + ((index >> i) & 1usize);
            let mut current_node_hash = Vec::with_capacity(
                self.hash_tree[next_node ^ 1usize].len() * zinc_const::bitlength::BYTE,
            );

            for i in self.hash_tree[next_node ^ 1usize].iter() {
                for j in (0..zinc_const::bitlength::BYTE).rev() {
                    let bit = (i >> j) & 1u8 == 1u8;
                    current_node_hash.push(bit);
                }
            }

            authentication_path.push(current_node_hash);
            current_node = next_node;
        }
        authentication_path.reverse();

        StorageLeaf::new(
            self.leaf_values[index].as_slice(),
            Some(authentication_path),
            self.depth,
        )
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn load(&self, index: BigInt) -> Result<StorageLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        Ok(self.leaf_with_auth(index))
    }

    fn store(
        &mut self,
        index: BigInt,
        values: Vec<Scalar<E>>,
    ) -> Result<StorageLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        let result = self.leaf_with_auth(index);

        self.leaf_values[index] = values;
        self.update_tree();

        Ok(result)
    }

    fn root_hash(&self) -> E::Fr {
        let mut hash_buffer = self.hash_tree[1].to_owned();
        hash_buffer
            .truncate(zinc_const::bitlength::SHA256_HASH_TRUNCATED / zinc_const::bitlength::BYTE);
        hash_buffer.resize(
            zinc_const::bitlength::SHA256_HASH / zinc_const::bitlength::BYTE,
            0,
        );

        let mut hash_le = Vec::with_capacity(hash_buffer.len());
        for i in hash_buffer.iter() {
            let mut current_byte: u8 = 0;
            for j in 0..zinc_const::bitlength::BYTE {
                current_byte <<= 1;
                current_byte += (i >> j) & 1u8;
            }
            hash_le.push(current_byte);
        }

        let mut hash_repr = <E::Fr as PrimeField>::Repr::default();
        hash_repr
            .read_le(hash_le.as_slice())
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
        E::Fr::from_repr(hash_repr).expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }

    fn depth(&self) -> usize {
        self.depth
    }
}
