//!
//! The database contract storage field INSERT model.
//!

use serde_json::Value as JsonValue;

use zksync::web3::types::Address;

///
/// The database contract storage field INSERT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address referencing `contracts.address`.
    pub address: Address,
    /// The field index in the contract storage.
    pub index: i16,
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, index: i16, name: String, value: JsonValue) -> Self {
        Self {
            address,
            index,
            name,
            value,
        }
    }
}
