//!
//! The cached contract data.
//!

use zinc_build::Contract as BuildContract;

///
/// The cached contract data.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The pre-built contract ready to be called.
    pub build: BuildContract,
    /// The contract address.
    pub eth_address: [u8; zinc_const::size::ETH_ADDRESS],
    /// The contract private key.
    pub private_key: [u8; zinc_const::size::ETH_PRIVATE_KEY],
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        build: BuildContract,
        eth_address: [u8; zinc_const::size::ETH_ADDRESS],
        private_key: [u8; zinc_const::size::ETH_PRIVATE_KEY],
    ) -> Self {
        Self {
            build,
            eth_address,
            private_key,
        }
    }
}
