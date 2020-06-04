use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Leaf<E: IEngine> {
    pub leaf_value: Vec<Option<Scalar<E>>>,
    pub leaf_value_hash: Vec<Option<bool>>,
    pub authentication_path: Vec<Vec<Option<bool>>>,
}
