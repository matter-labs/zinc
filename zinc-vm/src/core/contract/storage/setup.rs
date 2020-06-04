use num_bigint::BigInt;

use crate::error::RuntimeError;
use crate::gadgets::contract::merkle_tree::leaf::Leaf as MerkleTreeLeaf;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage {
    depth: usize,
}

impl Storage {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage {
    fn depth(&self) -> usize {
        self.depth
    }

    fn root_hash(&self) -> Option<E::Fr> {
        None
    }

    fn load(&self, _index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        unimplemented!()
    }

    fn store(
        &mut self,
        _index: &Option<BigInt>,
        _value: &[Option<Scalar<E>>],
    ) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        unimplemented!()
    }
}
