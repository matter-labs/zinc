//!
//! The contract resource `call` POST request.
//!

use std::iter::IntoIterator;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::Network;

use crate::transaction::Transaction;

///
/// The contract resource `call` POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract ETH address.
    pub address: Address,
    /// The name of the queried method.
    pub method: String,
    /// The network where the contract resides.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, method: String, network: Network) -> Self {
        Self {
            address,
            method,
            network,
        }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (
                "address",
                serde_json::to_string(&self.address)
                    .expect(zinc_const::panic::DATA_CONVERSION)
                    .replace("\"", ""),
            ),
            ("method", self.method),
            ("network", self.network.to_string()),
        ]
        .into_iter()
    }
}

///
/// The contract resource `call` POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON method input.
    pub arguments: JsonValue,
    /// The signed transaction which must be sent directly to zkSync.
    pub transaction: Transaction,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(arguments: JsonValue, transaction: Transaction) -> Self {
        Self {
            arguments,
            transaction,
        }
    }
}
