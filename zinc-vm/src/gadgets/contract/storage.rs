use std::borrow::BorrowMut;
use std::marker::PhantomData;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;

use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::leaf::LeafVariant;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::contract::merkle_tree::allocated_leaf::AllocatedLeaf;
use crate::gadgets::contract::merkle_tree::hasher::IHasher as IMerkleTreeHasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct StorageGadget<E: IEngine, S: IMerkleTree<E>, H: IMerkleTreeHasher<E>> {
    storage: S,
    root_hash: Scalar<E>,

    _pd: PhantomData<H>,
}

impl<E, S, H> StorageGadget<E, S, H>
where
    E: IEngine,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    pub fn new<CS>(mut cs: CS, storage: S) -> Result<Self, SynthesisError>
    where
        CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage.root_hash();
        let root_hash_variable = cs.alloc(|| "root hash variable", || Ok(root_hash_value))?;
        let root_hash = Scalar::<E>::new_unchecked_variable(
            Some(root_hash_value),
            root_hash_variable,
            zinc_types::ScalarType::Field,
        );

        Ok(StorageGadget {
            storage,
            root_hash,
            _pd: PhantomData,
        })
    }

    pub fn load<CS>(
        &self,
        mut cs: CS,
        index: Scalar<E>,
        _size: usize,
    ) -> Result<Vec<Scalar<E>>, Error>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let index = index
            .get_value()
            .map(|field| gadgets::scalar::fr_bigint::fr_to_bigint::<E>(&field, false))
            .expect(zinc_const::panic::TEST_DATA_VALID);
        let merkle_tree_leaf = self.storage.load(index)?;

        let leaf_value = match merkle_tree_leaf.leaf_values {
            LeafVariant::Array(array) => array,
            LeafVariant::Map { .. } => vec![],
        };
        let leaf_fields =
            AllocatedLeaf::alloc_leaf_fields(cs.namespace(|| "alloc leaf fields"), leaf_value)?;

        Ok(leaf_fields)
    }

    pub fn store<CS>(
        &mut self,
        mut cs: CS,
        index: Scalar<E>,
        values: LeafVariant<E>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let depth = self.storage.depth();
        let mut index_bits = index.get_bits_le(cs.namespace(|| "index into bits"))?;
        index_bits.truncate(depth);

        let _merkle_tree_leaf = self.storage.store(
            index
                .get_value()
                .map(|field| gadgets::scalar::fr_bigint::fr_to_bigint::<E>(&field, false))
                .expect(zinc_const::panic::TEST_DATA_VALID),
            values,
        )?;

        Ok(())
    }

    pub fn into_build(self) -> zinc_types::Value {
        let field_types = self.storage.types().to_owned();
        let fields = self
            .storage
            .into_values()
            .into_iter()
            .zip(field_types)
            .map(|(leaf, field)| {
                let value = match leaf {
                    LeafOutput::Array(array) => {
                        zinc_types::Value::from_flat_values(field.r#type, array.as_slice())
                    }
                    LeafOutput::Map(entries) => {
                        let (key_type, value_type) = match field.r#type {
                            zinc_types::Type::Map {
                                key_type,
                                value_type,
                            } => (*key_type, *value_type),
                            _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                        };

                        let mut values = Vec::with_capacity(entries.len());
                        for (key, value) in entries.into_iter() {
                            let key = zinc_types::Value::from_flat_values(
                                key_type.clone(),
                                key.as_slice(),
                            );
                            let value = zinc_types::Value::from_flat_values(
                                value_type.clone(),
                                value.as_slice(),
                            );
                            values.push((key, value));
                        }
                        zinc_types::Value::Map(values)
                    }
                };

                zinc_types::ContractFieldValue::new(
                    field.name,
                    value,
                    field.is_public,
                    field.is_implicit,
                )
            })
            .collect::<Vec<zinc_types::ContractFieldValue>>();

        zinc_types::Value::Contract(fields)
    }

    pub fn root_hash(&self) -> Result<Scalar<E>, Error> {
        Ok(self.root_hash.clone())
    }
}

impl<E, S, H> AsMut<S> for StorageGadget<E, S, H>
where
    E: IEngine,
    S: IMerkleTree<E>,
    H: IMerkleTreeHasher<E>,
{
    fn as_mut(&mut self) -> &mut S {
        self.storage.borrow_mut()
    }
}
