//!
//! The database contract INSERT new model.
//!

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync_types::AccountId;

///
/// The database contract INSERT new input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID.
    pub account_id: AccountId,

    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,

    /// The Zinc compiler version.
    pub zinc_version: String,
    /// The contract source code tree JSON representation.
    pub source_code: serde_json::Value,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract verifying key as a byte array.
    pub verifying_key: Vec<u8>,

    /// The contract ETH address.
    pub eth_address: Address,
    /// The contract private key.
    pub eth_private_key: H256,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: AccountId,

        name: String,
        version: String,
        instance: String,

        zinc_version: String,
        source_code: serde_json::Value,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,

        eth_address: Address,
        eth_private_key: H256,
    ) -> Self {
        Self {
            account_id,

            name,
            version,
            instance,

            zinc_version,
            source_code,
            bytecode,
            verifying_key,

            eth_address,
            eth_private_key,
        }
    }
}
