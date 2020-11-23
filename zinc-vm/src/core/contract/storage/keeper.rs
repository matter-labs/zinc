//!
//! The contract storage keeper trait.
//!

use num::BigInt;

use crate::error::Error;

///
/// The contract storage keeper trait.
///
pub trait IKeeper: Sync + Send {
    ///
    /// Fetches the contract storage instance by its ETH address.
    ///
    fn fetch(
        &self,
        eth_address: BigInt,
        field_types: Vec<zinc_build::ContractFieldType>,
    ) -> Result<zinc_build::Value, Error>;
}

///
/// The dummy keeper for places where loading is not implemented yet.
///
#[derive(Default)]
pub struct DummyKeeper {}

impl IKeeper for DummyKeeper {
    fn fetch(
        &self,
        _eth_address: BigInt,
        field_types: Vec<zinc_build::ContractFieldType>,
    ) -> Result<zinc_build::Value, Error> {
        Ok(zinc_build::Value::Contract(
            field_types
                .into_iter()
                .map(zinc_build::ContractFieldValue::new_from_type)
                .collect(),
        ))
    }
}
