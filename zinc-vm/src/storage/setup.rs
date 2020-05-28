use crate::gadgets::contracts::merkle_tree_storage::MerkleTreeStorage;
use crate::gadgets::merkle_tree_storage::MerkleTreeLeaf;
use crate::{Engine, Result};
use num_bigint::BigInt;

pub struct SetupStorage {
    depth: usize,
}

impl SetupStorage {
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }
}

impl<E: Engine> MerkleTreeStorage<E> for SetupStorage {
    fn depth(&self) -> usize {
        self.depth
    }

    fn root_hash(&self) -> Option<E::Fr> {
        None
    }

    fn load(&self, _index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>> {
        unimplemented!()
    }

    fn store(
        &mut self,
        _index: &Option<BigInt>,
        _value: &[Option<E::Fr>],
    ) -> Result<MerkleTreeLeaf<E>> {
        unimplemented!()
    }
}
