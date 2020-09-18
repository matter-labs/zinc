//!
//! The contract resource POST request.
//!

use std::iter::IntoIterator;

use num_bigint::BigUint;
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
    pub fn new(name: String, version: String, network: Network) -> Self {
        Self {
            name,
            version,
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
    pub arguments: JsonValue,
    /// The verifying key.
    pub verifying_key: Vec<u8>,
    /// The initial contract deposit transfer.
    pub transfer: Transfer,
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
        transfer: Transfer,
    ) -> Self {
        Self {
            source,
            bytecode,
            arguments,
            verifying_key,
            transfer,
        }
    }
}

///
/// The initial contract deposit transfer.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    /// The address where the initial deposit is made from.
    pub source_address: String,
    /// The private key of the account where the initial deposit is made from.
    pub source_private_key: String,
    /// The initial deposit amount.
    pub amount: BigUint,
}

impl Transfer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source_address: String, source_private_key: String, amount: BigUint) -> Self {
        Self {
            source_address,
            source_private_key,
            amount,
        }
    }
}
