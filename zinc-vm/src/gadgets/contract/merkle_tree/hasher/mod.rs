pub mod sha256;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;

use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub trait IHasher<E: IEngine>: Default {
    fn hash_width(&self) -> usize;

    fn leaf_value_hash<CS>(&self, cs: CS, leaf_value: &[Scalar<E>]) -> Result<Vec<Boolean>, Error>
    where
        CS: ConstraintSystem<E>;

    fn node_hash<CS>(
        &self,
        cs: CS,
        left_node: &[Boolean],
        right_node: &[Boolean],
    ) -> Result<Vec<Boolean>, Error>
    where
        CS: ConstraintSystem<E>;
}
