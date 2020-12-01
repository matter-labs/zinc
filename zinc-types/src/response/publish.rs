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
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address) -> Self {
        Self { address }
    }
}
