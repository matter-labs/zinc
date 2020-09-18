//!
//! The contract resource query PUT request.
//!

use std::iter::IntoIterator;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use zksync::zksync_models::node::AccountId;

use crate::Network;

///
/// The contract resource query PUT request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub account_id: AccountId,
    /// The name of the queried method. If not specified, the storage is returned.
    pub method: Option<String>,
    /// The network where the contract resides.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId, method: Option<String>, network: Network) -> Self {
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
        let mut result = Vec::with_capacity(3);
        result.push(("account_id", self.account_id.to_string()));
        if let Some(method) = self.method {
            result.push(("method", method));
        }
        result.push(("network", self.network.to_string()));
        result.into_iter()
    }
}

///
/// The contract resource query PUT request body.
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
