//!
//! The database contract INSERT one model.
//!

///
/// The database contract INSERT one input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID.
    pub account_id: zksync_types::AccountId,

    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: semver::Version,
    /// The contract instance name.
    pub instance: String,

    /// The contract ETH address.
    pub eth_address: zksync_types::Address,
    /// The contract private key.
    pub eth_private_key: zksync_types::H256,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: zksync_types::AccountId,

        name: String,
        version: semver::Version,
        instance: String,

        eth_address: zksync_types::Address,
        eth_private_key: zksync_types::H256,
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
