//!
//! The contract resource POST request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

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
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, name: String, version: String) -> Self {
        Self {
            contract_id,
            name,
            version,
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
    /// The JSON constructor input.
    pub arguments: JsonValue,
    /// The verifying key.
    pub verifying_key: String,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source: Source, arguments: JsonValue, verifying_key: String) -> Self {
        Self {
            source,
            arguments,
            verifying_key,
        }
    }
}
