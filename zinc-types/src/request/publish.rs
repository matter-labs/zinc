//!
//! The contract resource POST request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;
use serde::Serialize;

///
/// The contract resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The name of the uploaded contract.
    pub name: String,
    /// The version of the uploaded contract.
    pub version: semver::Version,
    /// The uploaded contract instance name.
    pub instance: String,
    /// The change-pubkey fee token.
    pub change_pubkey_fee_token: String,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        version: semver::Version,
        instance: String,
        change_pubkey_fee_token: String,
    ) -> Self {
        Self {
            name,
            version,
            instance,
            change_pubkey_fee_token,
        }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("name", self.name),
            ("version", self.version.to_string()),
            ("instance", self.instance),
            ("change_pubkey_fee_token", self.change_pubkey_fee_token),
        ]
        .into_iter()
    }
}

///
/// The contract resource POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The project data.
    pub project: zinc_project::Project,
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
        project: zinc_project::Project,
        bytecode: Vec<u8>,
        arguments: serde_json::Value,
        verifying_key: Vec<u8>,
    ) -> Self {
        Self {
            project,
            bytecode,
            arguments,
            verifying_key,
        }
    }
}
