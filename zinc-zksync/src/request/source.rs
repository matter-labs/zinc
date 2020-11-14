//!
//! The contract resource `source` GET request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;

///
/// The contract resource `source` GET request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract project name.
    pub name: String,
    /// The contract project version.
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
        let mut result = Vec::with_capacity(2);
        result.push(("name", self.name));
        result.push(("version", self.version.to_string()));
        result.into_iter()
    }
}
