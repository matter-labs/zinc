#![allow(clippy::type_complexity)]

use num::BigInt;

use zinc_build::Type as BuildType;

use crate::core::contract::storage::sha256;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Debug)]
pub struct Leaf<E: IEngine> {
    pub leaf_values: LeafVariant<E>,
    pub leaf_value_hash: Vec<bool>,
    pub authentication_path: Vec<Vec<bool>>,
}

#[derive(Debug, Clone)]
pub enum LeafVariant<E: IEngine> {
    Array(Vec<Scalar<E>>),
    Map {
        data: Vec<(Vec<Scalar<E>>, Vec<Scalar<E>>)>,
        key_size: usize,
        value_size: usize,
    },
}

#[derive(Debug, Clone)]
pub enum LeafInput {
    Array {
        r#type: BuildType,
        values: Vec<BigInt>,
    },
    Map {
        key_type: BuildType,
        value_type: BuildType,
        entries: Vec<(Vec<BigInt>, Vec<BigInt>)>,
    },
}

pub enum LeafOutput {
    Array(Vec<BigInt>),
    Map(Vec<(Vec<BigInt>, Vec<BigInt>)>),
}

impl<E: IEngine> Leaf<E> {
    pub fn new(
        leaf_values: LeafVariant<E>,
        authentication_path: Option<Vec<Vec<bool>>>,
        depth: usize,
    ) -> Self {
        Self {
            leaf_values: leaf_values.clone(),
            leaf_value_hash: {
                let mut hash = Vec::with_capacity(
                    zinc_const::bitlength::SHA256_HASH * zinc_const::bitlength::BYTE,
                );
                let values = match leaf_values {
                    LeafVariant::Array(array) => array,
                    LeafVariant::Map { .. } => vec![],
                };
                for i in sha256::leaf_value_hash::<E>(values).into_iter() {
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
