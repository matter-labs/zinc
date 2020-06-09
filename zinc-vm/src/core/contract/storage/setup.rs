use num_bigint::BigInt;
use num_traits::ToPrimitive;

use ff::Field;

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
            leaf_values: vec![vec![]; fields.len()],
        };

        for (index, field) in fields.into_iter().enumerate() {
            let values = field
                .to_scalar_types()
                .into_iter()
                .map(|r#type| Some(Scalar::<E>::new_constant_int(0, r#type)))
                .collect();
            result.leaf_values[index] = values;
        }

        result
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn depth(&self) -> usize {
        self.depth
    }

    fn root_hash(&self) -> Option<E::Fr> {
        Some(E::Fr::zero())
    }

    fn load(&self, index: BigInt) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        Ok(MerkleTreeLeaf::new(
            self.leaf_values[index].as_slice(),
            Some(self.depth),
        ))
    }

    fn store(
        &mut self,
        index: BigInt,
        value: Vec<Option<Scalar<E>>>,
    ) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        let index = index.to_usize().ok_or(RuntimeError::ExpectedUsize(index))?;

        self.leaf_values[index] = value;

        Ok(MerkleTreeLeaf::new(
            self.leaf_values[index].as_slice(),
            Some(self.depth),
        ))
    }
}
