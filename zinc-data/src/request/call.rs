//!
//! The contract resource call POST request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use crate::Network;

///
/// The contract resource call POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub contract_id: i64,
    /// The name of the queried method.
    pub method: String,
    /// The network where the contract resides.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, method: String, network: Network) -> Self {
        Self {
            contract_id,
            method,
            network,
        }
    }

    ///
    /// Converts the query into an iterable list of arguments.
    ///
    pub fn into_vec(self) -> Vec<(&'static str, String)> {
        vec![
            ("contract_id", self.contract_id.to_string()),
            ("method", self.method),
            ("network", self.network.to_string()),
        ]
    }
}

///
/// The contract resource call POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON method input.
    pub arguments: JsonValue,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(arguments: JsonValue) -> Self {
        Self { arguments }
    }
}
