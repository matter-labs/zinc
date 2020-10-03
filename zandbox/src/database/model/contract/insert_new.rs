//!
//! The database contract INSERT new model.
//!

use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync_types::AccountId;

///
/// The database contract INSERT new input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address.
    pub eth_address: Address,

    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,

    /// The Zinc compiler version.
    pub zinc_version: String,
    /// The contract source code tree JSON representation.
    pub source_code: JsonValue,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract verifying key as a byte array.
    pub verifying_key: Vec<u8>,

    /// The contract account ID.
    pub account_id: AccountId,
    /// The contract private key.
    pub eth_private_key: H256,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        eth_address: Address,

        name: String,
        version: String,
        instance: String,

        zinc_version: String,
        source_code: JsonValue,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,

        account_id: AccountId,
        eth_private_key: H256,
    ) -> Self {
        Self {
            eth_address,

            name,
            version,
            instance,

            zinc_version,
            source_code,
            bytecode,
            verifying_key,

            account_id,
            eth_private_key,
        }
    }
}
