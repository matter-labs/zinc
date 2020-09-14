//!
//! The database contract INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract INSERT input model.
///
pub struct Input {
    /// The contract account ID.
    pub contract_id: i64,
    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract source code tree JSON representation.
    pub source_code: JsonValue,
    /// The contract storage type JSON representation.
    pub storage_type: JsonValue,
    /// The contract verifying key as a byte array.
    pub verifying_key: Vec<u8>,
    /// The contract owner ETH address.
    pub eth_address: [u8; zinc_const::size::ETH_ADDRESS],
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract_id: i64,
        name: String,
        version: String,
        source_code: JsonValue,
        storage_type: JsonValue,
        verifying_key: Vec<u8>,
        eth_address: [u8; zinc_const::size::ETH_ADDRESS],
    ) -> Self {
        Self {
            contract_id,
            name,
            version,
            source_code,
            storage_type,
            verifying_key,
            eth_address,
        }
    }
}
