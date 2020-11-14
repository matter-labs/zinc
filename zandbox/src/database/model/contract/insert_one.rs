//!
//! The database contract INSERT one model.
//!

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync_types::AccountId;

///
/// The database contract INSERT one input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID.
    pub account_id: AccountId,

    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: semver::Version,
    /// The contract instance name.
    pub instance: String,

    /// The contract ETH address.
    pub eth_address: Address,
    /// The contract private key.
    pub eth_private_key: H256,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: AccountId,

        name: String,
        version: semver::Version,
        instance: String,

        eth_address: Address,
        eth_private_key: H256,
    ) -> Self {
        Self {
            account_id,

            name,
            version,
            instance,

            eth_address,
            eth_private_key,
        }
    }
}
