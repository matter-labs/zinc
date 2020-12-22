use num::BigInt;

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
#[allow(clippy::type_complexity)]
pub enum LeafVariant<E: IEngine> {
    Array(Vec<Scalar<E>>),
    Map {
        data: Vec<(Vec<Scalar<E>>, Vec<Scalar<E>>)>,
        key_size: usize,
        value_size: usize,
    },
}

impl<E: IEngine> LeafVariant<E> {
    pub fn new(input: LeafInput) -> Self {
        match input {
            LeafInput::Array { r#type, values } => Self::Array(
                r#type
                    .into_flat_scalar_types()
                    .into_iter()
                    .zip(values.into_iter())
                    .map(|(r#type, value)| {
                        Scalar::<E>::new_constant_bigint(value, r#type)
                            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    })
                    .collect::<Vec<Scalar<E>>>(),
            ),
            LeafInput::Map {
                key_type,
                value_type,
                entries,
            } => {
                let mut result = Vec::with_capacity(entries.len());
                for (key, value) in entries.into_iter() {
                    let key = key_type
                        .clone()
                        .into_flat_scalar_types()
                        .into_iter()
                        .zip(key.into_iter())
                        .map(|(r#type, value)| {
                            Scalar::<E>::new_constant_bigint(value, r#type)
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                        })
                        .collect::<Vec<Scalar<E>>>();

                    let value = value_type
                        .clone()
                        .into_flat_scalar_types()
                        .into_iter()
                        .zip(value.into_iter())
                        .map(|(r#type, value)| {
                            Scalar::<E>::new_constant_bigint(value, r#type)
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                        })
                        .collect::<Vec<Scalar<E>>>();

                    result.push((key, value));
                }
                Self::Map {
                    data: result,
                    key_size: key_type.size(),
                    value_size: value_type.size(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LeafInput {
    Array {
        r#type: zinc_types::Type,
        values: Vec<BigInt>,
    },
    Map {
        key_type: zinc_types::Type,
        value_type: zinc_types::Type,
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
