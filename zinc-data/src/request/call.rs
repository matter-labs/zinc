//!
//! The contract resource call POST request.
//!

use std::iter::IntoIterator;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use zksync::zksync_models::node::tx::PackedEthSignature;
use zksync::zksync_models::node::tx::Transfer;
use zksync::zksync_models::node::AccountId;
use zksync::Network;

///
/// The contract resource call POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub account_id: AccountId,
    /// The name of the queried method.
    pub method: String,
    /// The network where the contract resides.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId, method: String, network: Network) -> Self {
        Self {
            account_id,
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
            ("account_id", self.account_id.to_string()),
            ("method", self.method),
            ("network", self.network.to_string()),
        ]
        .into_iter()
    }
}

///
/// The contract resource call POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON method input.
    pub arguments: JsonValue,
    /// The signed transfer which must be sent directly to zkSync.
    pub transfer: Transfer,
    /// The transaction ETH signature.
    pub signature: PackedEthSignature,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(arguments: JsonValue, transfer: Transfer, signature: PackedEthSignature) -> Self {
        Self {
            arguments,
            transfer,
            signature,
        }
    }
}
