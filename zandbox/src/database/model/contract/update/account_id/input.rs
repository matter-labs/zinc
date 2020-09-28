//!
//! The database contract UPDATE account ID input model.
//!

use zksync::web3::types::Address;
use zksync::zksync_models::AccountId;

///
/// The database contract UPDATE account ID input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address.
    pub address: Address,

    /// The contract account ID.
    pub account_id: AccountId,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, account_id: AccountId) -> Self {
        Self {
            address,

            account_id,
        }
    }
}
