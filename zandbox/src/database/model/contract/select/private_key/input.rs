//!
//! The database contract private key SELECT input model.
//!

///
/// The database contract private key SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address.
    pub eth_address: [u8; zinc_const::size::ETH_ADDRESS],
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(eth_address: [u8; zinc_const::size::ETH_ADDRESS]) -> Self {
        Self { eth_address }
    }
}
