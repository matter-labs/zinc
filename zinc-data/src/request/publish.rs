//!
//! The contract resource POST request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use crate::network::Network;
use crate::source::Source;

///
/// The contract resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub contract_id: i64,
    /// The name of the uploaded contract.
    pub name: String,
    /// The version of the uploaded contract.
    pub version: String,
    /// The network where the contract must be uploaded to.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, name: String, version: String, network: Network) -> Self {
        Self {
            contract_id,
            name,
            version,
            network,
        }
    }

    ///
    /// Converts the query into an iterable list of arguments.
    ///
    pub fn into_vec(self) -> Vec<(&'static str, String)> {
        vec![
            ("contract_id", self.contract_id.to_string()),
            ("name", self.name),
            ("version", self.version),
            ("network", self.network.to_string()),
        ]
    }
}

///
/// The contract resource POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON source code tree.
    pub source: Source,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The JSON constructor input.
    pub arguments: JsonValue,
    /// The verifying key.
    pub verifying_key: Vec<u8>,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        source: Source,
        bytecode: Vec<u8>,
        arguments: JsonValue,
        verifying_key: Vec<u8>,
    ) -> Self {
        Self {
            source,
            bytecode,
            arguments,
            verifying_key,
        }
    }
}
