//!
//! The array-vector conversion tools.
//!

use zksync::web3::types::H160;
use zksync::web3::types::H256;

pub fn eth_address_from_vec(vector: Vec<u8>) -> H160 {
    let mut array = [0; zinc_const::size::ETH_ADDRESS];
    for (index, byte) in vector.into_iter().enumerate() {
        array[index] = byte;
    }
    array.into()
}

pub fn eth_private_key_from_vec(vector: Vec<u8>) -> H256 {
    let mut array = [0; zinc_const::size::ETH_PRIVATE_KEY];
    for (index, byte) in vector.into_iter().enumerate() {
        array[index] = byte;
    }
    array.into()
}
