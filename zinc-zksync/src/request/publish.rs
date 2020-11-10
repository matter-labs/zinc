//!
//! The contract resource POST request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;
use serde::Serialize;

use zinc_source::Source;

use zksync::Network;

///
/// The contract resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The name of the uploaded contract.
    pub name: String,
    /// The version of the uploaded contract.
    pub version: String,
    /// The uploaded contract instance name.
    pub instance: String,
    /// The network where the contract must be uploaded to.
    pub network: Network,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, version: String, instance: String, network: Network) -> Self {
        Self {
            name,
            version,
            instance,
            network,
        }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("name", self.name),
            ("version", self.version),
            ("instance", self.instance),
            ("network", self.network.to_string()),
        ]
        .into_iter()
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
    pub arguments: serde_json::Value,
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
        arguments: serde_json::Value,
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
