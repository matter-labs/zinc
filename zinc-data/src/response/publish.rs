//!
//! The contract resource POST request.
//!

use serde_derive::Serialize;

use zksync::web3::types::H160;
use zksync::zksync_models::node::AccountId;

///
/// The contract resource POST response body.
///
#[derive(Debug, Serialize)]
pub struct Body {
    /// The contract zkSync account ID.
    pub account_id: AccountId,
    /// The contract address.
    pub address: H160,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId, address: H160) -> Self {
        Self {
            account_id,
            address,
        }
    }
}
