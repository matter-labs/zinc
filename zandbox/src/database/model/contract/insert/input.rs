//!
//! The database contract instance INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract instance INSERT input model.
///
pub struct Input {
    /// The contract account ID.
    pub account_id: i64,
    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract source code tree JSON representation.
    pub source_code: JsonValue,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract storage type JSON representation.
    pub storage_type: JsonValue,
    /// The contract verifying key as a byte array.
    pub verifying_key: Vec<u8>,
    /// The contract owner ETH address.
    pub eth_address: Vec<u8>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: i64,
        name: String,
        version: String,
        source_code: JsonValue,
        bytecode: Vec<u8>,
        storage_type: JsonValue,
        verifying_key: Vec<u8>,
        eth_address: Vec<u8>,
    ) -> Self {
        Self {
            account_id,
            name,
            version,
            source_code,
            bytecode,
            storage_type,
            verifying_key,
            eth_address,
        }
    }
}
