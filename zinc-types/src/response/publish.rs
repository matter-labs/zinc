//!
//! The contract resource POST request.
//!

use serde::Deserialize;
use serde::Serialize;

use zksync_types::Address;

///
/// The contract resource POST response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The contract address.
    pub address: Address,
    /// The fee required to execute the change-pubkey transaction.
    pub change_pubkey_fee: num::BigUint,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, change_pubkey_fee: num::BigUint) -> Self {
        Self {
            address,
            change_pubkey_fee,
        }
    }
}
