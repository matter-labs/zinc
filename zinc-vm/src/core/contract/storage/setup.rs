use num::bigint::ToBigInt;
use num::BigInt;
use num::ToPrimitive;

use franklin_crypto::bellman::pairing::ff::Field;

use crate::core::contract::storage::leaf::Leaf;
use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::leaf::LeafVariant;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub struct Storage<E: IEngine> {
    field_types: Vec<zinc_types::ContractFieldType>,
    leaf_values: Vec<Vec<Scalar<E>>>,
    depth: usize,
}

impl<E: IEngine> IMerkleTree<E> for Storage<E> {
    fn from_evaluation_stack(
        field_types: Vec<zinc_types::ContractFieldType>,
        _values: Vec<Scalar<E>>,
    ) -> Result<Self, Error> {
        let depth = (field_types.len() as f64).log2().ceil() as usize;
        let leaf_values_count = 1 << depth;

        let mut result = Self {
            field_types: field_types.clone(),
            leaf_values: vec![vec![]; leaf_values_count],
            depth,
        };

        for (index, field) in field_types.into_iter().enumerate() {
            let values = field
                .r#type
                .into_flat_scalar_types()
                .into_iter()
                .map(|r#type| Scalar::<E>::new_constant_usize(0, r#type))
                .collect();
            result.leaf_values[index] = values;
        }

        Ok(result)
    }

    fn from_build(
        field_types: Vec<zinc_types::ContractFieldType>,
        _value: zinc_types::Value,
    ) -> Result<Self, Error> {
        let depth = (field_types.len() as f64).log2().ceil() as usize;
        let leaf_values_count = 1 << depth;

        let mut result = Self {
            field_types: field_types.clone(),
            leaf_values: vec![vec![]; leaf_values_count],
            depth,
        };

        for (index, field) in field_types.into_iter().enumerate() {
            let values = field
                .r#type
                .into_flat_scalar_types()
                .into_iter()
                .map(|r#type| Scalar::<E>::new_constant_usize(0, r#type))
                .collect();
            result.leaf_values[index] = values;
        }

        Ok(result)
    }

    fn load(&self, index: BigInt) -> Result<Leaf<E>, Error> {
        let index = index.to_usize().ok_or(Error::ExpectedUsize(index))?;

        Ok(Leaf::new(
            LeafVariant::Array(self.leaf_values[index].to_owned()),
            None,
            self.depth,
        ))
    }

    fn store(&mut self, index: BigInt, value: LeafVariant<E>) -> Result<(), Error> {
        let index = index.to_usize().ok_or(Error::ExpectedUsize(index))?;

        self.leaf_values[index] = match value {
            LeafVariant::Array(array) => array,
            LeafVariant::Map { .. } => vec![],
        };

        Ok(())
    }

    fn into_values(self) -> Vec<LeafOutput> {
        self.leaf_values
            .into_iter()
            .map(|field| {
                LeafOutput::Array(
                    field
                        .into_iter()
                        .map(|scalar| {
                            Scalar::to_bigint(&scalar)
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn types(&self) -> &[zinc_types::ContractFieldType] {
        self.field_types.as_slice()
    }

    fn root_hash(&self) -> E::Fr {
        E::Fr::zero()
    }

    fn depth(&self) -> usize {
        self.depth
    }
}
