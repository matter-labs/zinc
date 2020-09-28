//!
//! The cached contract data.
//!

use zksync::web3::types::H256;
use zksync::zksync_models::AccountId;

use zinc_build::Contract as BuildContract;

///
/// The cached contract data.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The pre-built contract ready to be called.
    pub build: BuildContract,
    /// The contract ETH private key.
    pub eth_private_key: H256,
    /// The contract zkSync account ID. Is set when the change-pubkey transaction is executed.
    pub account_id: Option<AccountId>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: BuildContract, eth_private_key: H256) -> Self {
        Self {
            build,
            eth_private_key,
            account_id: None,
        }
    }

    ///
    /// Sets the zkSync account ID.
    ///
    pub fn set_account_id(&mut self, account_id: AccountId) {
        self.account_id = Some(account_id);
    }
}
