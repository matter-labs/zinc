use num::bigint::ToBigInt;
use num::BigInt;
use num::ToPrimitive;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;

use crate::core::contract::storage::leaf::Leaf;
use crate::core::contract::storage::leaf::LeafInput;
use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::leaf::LeafVariant;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage<E: IEngine> {
    hash_tree: Vec<Vec<u8>>,
    leaf_values: Vec<LeafVariant<E>>,
    depth: usize,
}

impl<E: IEngine> Storage<E> {
    pub fn new(input: Vec<LeafInput>) -> Self {
        let depth = (input.len() as f64).log2().ceil() as usize;
        let hash_tree_size = 1 << (depth + 1);
        let leaf_values_size = 1 << depth;

        let mut leaf_values = input
            .into_iter()
            .map(|leaf| match leaf {
                LeafInput::Array { r#type, values } => LeafVariant::Array(
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
                    LeafVariant::Map {
                        data: result,
                        key_size: key_type.size(),
                        value_size: value_type.size(),
                    }
                }
            })
            .collect::<Vec<LeafVariant<E>>>();
        leaf_values.resize(leaf_values_size, LeafVariant::Array(vec![]));

        Self {
            hash_tree: vec![vec![]; hash_tree_size],
            leaf_values,
            depth,
        }
    }
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn load(&self, index: BigInt) -> Result<Leaf<E>, Error> {
        let index = index.to_usize().ok_or(Error::ExpectedUsize(index))?;

        Ok(Leaf::new(
            self.leaf_values[index].to_owned(),
            None,
            self.depth,
        ))
    }

    fn store(&mut self, index: BigInt, value: LeafVariant<E>) -> Result<(), Error> {
        let index = index.to_usize().ok_or(Error::ExpectedUsize(index))?;

        self.leaf_values[index] = value;

        Ok(())
    }

    fn into_values(self) -> Vec<LeafOutput> {
        self.leaf_values
            .into_iter()
            .map(|leaf| match leaf {
                LeafVariant::Array(array) => LeafOutput::Array(
                    array
                        .into_iter()
                        .map(|scalar| {
                            Scalar::to_bigint(&scalar)
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                        })
                        .collect(),
                ),
                LeafVariant::Map { data, .. } => LeafOutput::Map(
                    data.into_iter()
                        .map(|(key, value)| {
                            let key: Vec<BigInt> = key
                                .into_iter()
                                .map(|scalar| {
                                    Scalar::to_bigint(&scalar)
                                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                                })
                                .collect();
                            let value: Vec<BigInt> = value
                                .into_iter()
                                .map(|scalar| {
                                    Scalar::to_bigint(&scalar)
                                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                                })
                                .collect();
                            (key, value)
                        })
                        .collect(),
                ),
            })
            .collect()
    }

    fn root_hash(&self) -> E::Fr {
        let mut hash_buffer = self.hash_tree[1].to_owned();
        hash_buffer.truncate(zinc_const::size::SHA256_HASH - 1);
        hash_buffer.resize(zinc_const::size::SHA256_HASH, 0);

        let mut hash_le = Vec::with_capacity(hash_buffer.len());
        for i in hash_buffer.iter() {
            let mut current_byte: u8 = 0;
            for j in 0..zinc_const::bitlength::BYTE {
                current_byte <<= 1;
                current_byte += (i >> j) & 1u8;
            }
            hash_le.push(current_byte);
        }

        let mut hash_repr = <E::Fr as PrimeField>::Repr::default();
        hash_repr
            .read_le(hash_le.as_slice())
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
        E::Fr::from_repr(hash_repr).expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }

    fn depth(&self) -> usize {
        self.depth
    }
}
