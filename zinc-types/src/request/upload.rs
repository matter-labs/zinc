//!
//! The project resource POST request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;
use serde::Serialize;

///
/// The project resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The name of the uploaded contract.
    pub name: String,
    /// The version of the uploaded contract.
    pub version: semver::Version,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, version: semver::Version) -> Self {
        Self { name, version }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![("name", self.name), ("version", self.version.to_string())].into_iter()
    }
}

///
/// The project resource POST request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The project data.
    pub project: zinc_project::Project,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The verifying key.
    pub verifying_key: Vec<u8>,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(project: zinc_project::Project, bytecode: Vec<u8>, verifying_key: Vec<u8>) -> Self {
        Self {
            project,
            bytecode,
            verifying_key,
        }
    }
}
