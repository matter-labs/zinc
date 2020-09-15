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

    /// The Zinc compiler version.
    pub zinc_version: String,
    /// The contract source code tree JSON representation.
    pub source_code: JsonValue,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,

    /// The contract verifying key as a byte array.
    pub verifying_key: Vec<u8>,
    /// The contract ETH address.
    pub eth_address: [u8; zinc_const::size::ETH_ADDRESS],
    /// The contract public key.
    pub public_key: [u8; zinc_const::size::ETH_PUBLIC_KEY],
    /// The contract private key.
    pub private_key: [u8; zinc_const::size::ETH_PRIVATE_KEY],
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

        zinc_version: String,
        source_code: JsonValue,
        bytecode: Vec<u8>,

        verifying_key: Vec<u8>,
        eth_address: [u8; zinc_const::size::ETH_ADDRESS],
        public_key: [u8; zinc_const::size::ETH_PUBLIC_KEY],
        private_key: [u8; zinc_const::size::ETH_PRIVATE_KEY],
    ) -> Self {
        Self {
            contract_id,

            name,
            version,

            zinc_version,
            source_code,
            bytecode,

            verifying_key,
            eth_address,
            public_key,
            private_key,
        }
    }
}
