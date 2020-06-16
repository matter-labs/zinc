use num_bigint::BigInt;
use num_traits::ToPrimitive;

use franklin_crypto::bellman::pairing::ff::Field;

use zinc_bytecode::DataType;

use crate::core::contract::storage::leaf::Leaf as StorageLeaf;
use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage<E: IEngine> {
    leaf_values: Vec<Vec<Scalar<E>>>,
    depth: usize,
}

impl<E: IEngine> Storage<E> {
    pub fn new(values: Vec<DataType>) -> Self {
        let depth = (values.len() as f64).log2().ceil() as usize;
        let leaf_values_count = 1 << depth;

        let mut result = Self {
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

        result
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn load(&self, index: BigInt) -> Result<StorageLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        Ok(StorageLeaf::new(
            self.leaf_values[index].as_slice(),
            None,
            self.depth,
        ))
    }

    fn store(
        &mut self,
        index: BigInt,
        value: Vec<Scalar<E>>,
    ) -> Result<StorageLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        self.leaf_values[index] = value;

        Ok(StorageLeaf::new(
            self.leaf_values[index].as_slice(),
            None,
            self.depth,
        ))
    }

    fn root_hash(&self) -> E::Fr {
        E::Fr::zero()
    }

    fn depth(&self) -> usize {
        self.depth
    }
}
