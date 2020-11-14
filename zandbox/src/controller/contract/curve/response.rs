//!
//! The contract resource GET `curve` response.
//!

use serde::Serialize;

use zksync::web3::types::Address;

///
/// The contract resource GET `curve` response body.
///
pub type Body = Vec<Instance>;

///
/// The contract resource GET `curve` response instance.
///
#[derive(Debug, Serialize)]
pub struct Instance {
    /// The contract ETH address.
    pub address: Address,
    /// The project name.
    pub name: String,
    /// The project version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,
}

impl Instance {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, name: String, version: String, instance: String) -> Self {
        Self {
            address,
            name,
            version,
            instance,
        }
    }
}
