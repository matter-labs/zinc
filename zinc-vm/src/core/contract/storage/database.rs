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
    field_types: Vec<zinc_types::ContractFieldType>,
    hash_tree: Vec<Vec<u8>>,
    leaf_values: Vec<LeafVariant<E>>,
    depth: usize,
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn from_evaluation_stack(
        field_types: Vec<zinc_types::ContractFieldType>,
        mut values: Vec<Scalar<E>>,
    ) -> Result<Self, Error> {
        let mut storage_leaves = Vec::with_capacity(field_types.len());
        for field_type in field_types.iter() {
            let leaf = match field_type.r#type {
                zinc_types::Type::Map {
                    ref key_type,
                    ref value_type,
                } => LeafInput::Map {
                    key_type: *key_type.to_owned(),
                    value_type: *value_type.to_owned(),
                    entries: vec![],
                },
                ref r#type => {
                    let values: Vec<BigInt> = values
                        .drain(..r#type.size())
                        .map(|value| value.to_bigint().expect(zinc_const::panic::DATA_CONVERSION))
                        .collect();
                    LeafInput::Array {
                        r#type: r#type.to_owned(),
                        values,
                    }
                }
            };

            storage_leaves.push(leaf);
        }

        let depth = (storage_leaves.len() as f64).log2().ceil() as usize;
        let hash_tree_size = 1 << (depth + 1);

        let leaf_values = storage_leaves
            .into_iter()
            .map(LeafVariant::new)
            .collect::<Vec<LeafVariant<E>>>();

        Ok(Self {
            field_types,
            hash_tree: vec![vec![]; hash_tree_size],
            leaf_values,
            depth,
        })
    }

    fn from_build(
        field_types: Vec<zinc_types::ContractFieldType>,
        value: zinc_types::Value,
    ) -> Result<Self, Error> {
        let storage_leaves = match value {
            zinc_types::Value::Contract(fields) => fields
                .into_iter()
                .enumerate()
                .map(|(index, field)| {
                    let r#type = field_types[index].r#type.to_owned();

                    match field.value {
                        zinc_types::Value::Map(map) => {
                            let (key_type, value_type) = match r#type {
                                zinc_types::Type::Map {
                                    key_type,
                                    value_type,
                                } => (*key_type, *value_type),
                                _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                            };

                            let entries = map
                                .into_iter()
                                .map(|(key, value)| {
                                    (key.into_flat_values(), value.into_flat_values())
                                })
                                .collect();
                            LeafInput::Map {
                                key_type,
                                value_type,
                                entries,
                            }
                        }
                        value => {
                            let mut values = value.into_flat_values();
                            values.reverse();
                            LeafInput::Array { r#type, values }
                        }
                    }
                })
                .collect::<Vec<LeafInput>>(),
            _ => return Err(Error::InvalidStorageValue),
        };

        let depth = (storage_leaves.len() as f64).log2().ceil() as usize;
        let hash_tree_size = 1 << (depth + 1);

        let leaf_values = storage_leaves
            .into_iter()
            .map(LeafVariant::new)
            .collect::<Vec<LeafVariant<E>>>();

        Ok(Self {
            field_types,
            hash_tree: vec![vec![]; hash_tree_size],
            leaf_values,
            depth,
        })
    }

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

    fn types(&self) -> &[zinc_types::ContractFieldType] {
        self.field_types.as_slice()
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
