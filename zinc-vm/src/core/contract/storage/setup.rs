use num_bigint::BigInt;
use num_traits::ToPrimitive;

use zinc_bytecode::DataType;

use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::leaf::Leaf as MerkleTreeLeaf;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage<E: IEngine> {
    depth: usize,
    leaf_values: Vec<Vec<Option<Scalar<E>>>>,
}

impl<E: IEngine> Storage<E> {
    pub fn new(fields: Vec<DataType>) -> Self {
        let depth = (fields.len() as f64).log2().ceil() as usize;

        let mut result = Self {
            depth,
            leaf_values: vec![vec![]; 1 << depth],
        };

        for (index, field) in fields.into_iter().enumerate() {
            result.leaf_values[index] = vec![None; field.to_scalar_types().len()];
        }

        result
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn depth(&self) -> usize {
        self.depth
    }

    fn root_hash(&self) -> Option<E::Fr> {
        None
    }

    fn load(&self, index: BigInt) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        let result = MerkleTreeLeaf::<E> {
            leaf_values: self.leaf_values[index].to_owned(),
            leaf_value_hash: vec![],
            authentication_path: vec![],
        };

        Ok(result)
    }

    fn store(
        &mut self,
        index: BigInt,
        value: Vec<Option<Scalar<E>>>,
    ) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        self.leaf_values[index] = value;

        let result = MerkleTreeLeaf::<E> {
            leaf_values: self.leaf_values[index].to_owned(),
            leaf_value_hash: vec![],
            authentication_path: vec![],
        };

        Ok(result)
    }
}
