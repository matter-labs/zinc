//!
//! The contract resource `query` PUT request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;

use zksync::Network;
use zksync_types::Address;

///
/// The contract resource `query` PUT request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract ETH address.
    pub address: Address,
    /// The name of the queried method. If not specified, the storage is returned.
    pub method: Option<String>,
    /// The network where the contract resides.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address, method: Option<String>, network: Network) -> Self {
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
        let mut result = Vec::with_capacity(3);
        result.push((
            "address",
            serde_json::to_string(&self.address)
                .expect(zinc_const::panic::DATA_CONVERSION)
                .replace("\"", ""),
        ));
        if let Some(method) = self.method {
            result.push(("method", method));
        }
        result.push(("network", self.network.to_string()));
        result.into_iter()
    }
}

///
/// The contract resource `query` PUT request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON method input. Required for querying methods.
    pub arguments: Option<JsonValue>,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(arguments: Option<JsonValue>) -> Self {
        Self { arguments }
    }
}
