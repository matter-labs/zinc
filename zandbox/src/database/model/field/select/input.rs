//!
//! The database contract storage field SELECT input model.
//!

use zksync::web3::types::Address;

///
/// The database contract storage field SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address referencing `contracts.address`.
    pub address: Address,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address) -> Self {
        Self { address }
    }
}
