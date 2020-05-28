use num_bigint::BigInt;

use crate::error::RuntimeError;
use crate::gadgets::contract::MerkleTreeLeaf;
use crate::gadgets::contract::MerkleTreeStorage;
use crate::Engine;

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

    fn load(&self, _index: &Option<BigInt>) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        unimplemented!()
    }

    fn store(
        &mut self,
        _index: &Option<BigInt>,
        _value: &[Option<E::Fr>],
    ) -> Result<MerkleTreeLeaf<E>, RuntimeError> {
        unimplemented!()
    }
}