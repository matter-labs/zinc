//!
//! The Zinc source code JSON representation.
//!

pub(crate) mod network;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod source;

pub use self::network::Network;
pub use self::request::call::Body as CallRequestBody;
pub use self::request::call::Query as CallRequestQuery;
pub use self::request::publish::Body as PublishRequestBody;
pub use self::request::publish::Query as PublishRequestQuery;
pub use self::request::publish::Transfer as PublishRequestBodyTransfer;
pub use self::request::query::Body as QueryRequestBody;
pub use self::request::query::Query as QueryRequestQuery;
pub use self::response::publish::Body as PublishResponseBody;
pub use self::source::directory::Directory;
pub use self::source::error::Error as SourceError;
pub use self::source::file::File;
pub use self::source::Source;

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
