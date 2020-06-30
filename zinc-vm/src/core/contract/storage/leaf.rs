use crate::core::contract::storage::sha256;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Debug)]
pub struct Leaf<E: IEngine> {
    pub leaf_values: Vec<Scalar<E>>,
    pub leaf_value_hash: Vec<bool>,
    pub authentication_path: Vec<Vec<bool>>,
}

impl<E: IEngine> Leaf<E> {
    pub fn new(
        values: &[Scalar<E>],
        authentication_path: Option<Vec<Vec<bool>>>,
        depth: usize,
    ) -> Self {
        Self {
            leaf_values: values.to_owned(),
            leaf_value_hash: {
                let mut hash = Vec::with_capacity(
                    zinc_const::bitlength::SHA256_HASH * zinc_const::bitlength::BYTE,
                );
                for i in sha256::leaf_value_hash::<E>(values.to_owned()).into_iter() {
                    for j in (0..zinc_const::bitlength::BYTE).rev() {
                        let bit = ((i >> j) & 1u8) == 1u8;
                        hash.push(bit);
                    }
                }
                hash
            },
            authentication_path: authentication_path
                .unwrap_or_else(|| vec![vec![false; zinc_const::bitlength::SHA256_HASH]; depth]),
        }
    }
}
