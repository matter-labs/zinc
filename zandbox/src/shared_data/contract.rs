//!
//! The cached contract data.
//!

use zksync::web3::types::H160;
use zksync::web3::types::H256;

use zinc_build::Contract as BuildContract;

///
/// The cached contract data.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The pre-built contract ready to be called.
    pub build: BuildContract,
    /// The contract ETH address.
    pub eth_address: H160,
    /// The contract ETH private key.
    pub eth_private_key: H256,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: BuildContract, eth_address: H160, eth_private_key: H256) -> Self {
        Self {
            build,
            eth_address,
            eth_private_key,
        }
    }
}
