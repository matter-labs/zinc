//!
//! The contract resource POST request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use zksync::web3::types::Address;

///
/// The contract resource POST response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The contract address.
    pub address: Address,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address) -> Self {
        Self { address }
    }
}
